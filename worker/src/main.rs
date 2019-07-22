use std::thread;
use std::thread::JoinHandle;
use std::time;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
    mpsc,
    mpsc::{Sender, Receiver}
};

use reqwest;

use shared::models::{
    WorkerInfo, WorkerAction
};

enum WorkerCommand {
    Stop,
    Start,
    Update(u32),
    Quit
}

fn main() {
    let stop_flag = Arc::new(AtomicBool::default());
    let (tx, rx) = mpsc::channel();

    let us_thread_handle = update_status(stop_flag.clone(), tx);
    let p_thread_handle = process(rx);

    println!("Press ENTER to exit...");

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Read input failed");

    stop_flag.store(true, Ordering::Relaxed);

    us_thread_handle.join().unwrap();
    p_thread_handle.join().unwrap();
}

fn update_status(stop_flag: Arc<AtomicBool>, tx: Sender<WorkerCommand>) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            if stop_flag.load(Ordering::Relaxed) {
                tx.send(WorkerCommand::Quit).unwrap();
                break;
            }

            let mut prev_status = WorkerInfo::default();

            match reqwest::get("http://127.0.0.1:8081/bot-runner/state") {
                Ok(ref mut response) if response.status().is_success() => {
                    let status: WorkerInfo = response.json().unwrap();

                    if status.version > prev_status.version {
                        tx.send(WorkerCommand::Stop).unwrap();

                        if status.update_version > prev_status.update_version {
                            tx.send(WorkerCommand::Update(status.update_version)).unwrap();
                        }

                        match status.action {
                            WorkerAction::Start => {
                                tx.send(WorkerCommand::Start).unwrap();
                            },
                            WorkerAction::Stop => (),
                            WorkerAction::Update => ()
                        };
                    }
                    
                    prev_status = status;
                },
                _ => {
                    eprintln!("Error updating status");
                }
            }

            thread::sleep(time::Duration::from_secs(10));
        }

        println!("Update state stopped");
    })
}

fn process(rx: Receiver<WorkerCommand>) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let cmd = rx.recv().unwrap();
            match cmd {
                WorkerCommand::Quit => break,
                WorkerCommand::Start => {
                    dbg!("Started");
                },
                WorkerCommand::Stop => {
                    dbg!("Stopped");
                }
                WorkerCommand::Update(update_version) => {
                    download_update(update_version);
                }
            }
        }

        println!("Process stopped");
    })
}

fn download_update(update_version: u32) {
    let file_name = format!("update{}.zip", update_version);
    let mut file = std::fs::File::create(file_name).unwrap();
    let bytes_read = reqwest::get("http://127.0.0.1:8081/bot-runner/update")
        .unwrap()
        .copy_to(&mut file)
        .unwrap();

    
}