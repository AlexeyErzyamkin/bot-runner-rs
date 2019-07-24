use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::thread::JoinHandle;
use std::time;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
    mpsc,
    mpsc::{Sender, Receiver}
};
use std::process::{
    Command, Child
};
use std::mem;

use reqwest;
use reqwest::Url;

use serde::Deserialize;

use zip::{
    ZipArchive,
    read::ZipFile
};

use shared;
use shared::models::{
    WorkerInfo, WorkerAction, StartInfo
};

#[derive(Deserialize)]
struct WorkerConfig {
    addr: String
}

enum WorkerCommand {
    Stop,
    Start(StartInfo),
    Update(u32),
    Quit
}

fn main() -> io::Result<()> {
    let config_path = Path::new("./data/worker_config.json");
    let config: WorkerConfig = shared::read_config(config_path)
        .map_err(|e| {
            eprintln!("Error reading config");
            e
        })?;

    let stop_flag = Arc::new(AtomicBool::default());
    let (tx, rx) = mpsc::channel();

    let download_url = Url::parse(&config.addr).unwrap().join("/bot-runner/update").unwrap();
    let state_url = Url::parse(&config.addr).unwrap().join("/bot-runner/state").unwrap();

    let us_thread_handle = update_status(stop_flag.clone(), tx, state_url);
    let p_thread_handle = process(rx, download_url);

    println!("Press ENTER to exit...");

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Read input failed");

    stop_flag.store(true, Ordering::Relaxed);

    us_thread_handle.join().unwrap();
    p_thread_handle.join().unwrap();

    Ok(())
}

fn update_status(stop_flag: Arc<AtomicBool>, tx: Sender<WorkerCommand>, addr: Url) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut prev_status = WorkerInfo::default();

        loop {
            if stop_flag.load(Ordering::Relaxed) {
                tx.send(WorkerCommand::Quit).unwrap();
                break;
            }

            match reqwest::get(addr.clone()) {
                Ok(ref mut response) if response.status().is_success() => {
                    let status: WorkerInfo = response.json().unwrap();

                    if status.version > prev_status.version {
                        tx.send(WorkerCommand::Stop).unwrap();

                        if status.update_version > prev_status.update_version {
                            tx.send(WorkerCommand::Update(status.update_version)).unwrap();
                        }

                        match status.action {
                            WorkerAction::Start => {
                                tx.send(WorkerCommand::Start(status.start_info.clone())).unwrap();
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

fn process(rx: Receiver<WorkerCommand>, download_url: Url) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut started_process = None;

        loop {
            let cmd = rx.recv().unwrap();
            match cmd {
                WorkerCommand::Quit => break,
                WorkerCommand::Start(start_info) => {
                    let mut path = PathBuf::from("./data/unpacked");
                    if start_info.current_dir.len() > 0 {
                        path.push(start_info.current_dir);
                    }

                    let path = path.canonicalize().unwrap();

                    let command_path = if &start_info.command[..2] == "./" {
                        let mut command_path = PathBuf::from(&path);
                        command_path.push(start_info.command);

                        command_path
                    } else {
                        PathBuf::from(start_info.command)
                    };

                    println!("Starting command '{:?}' from '{:?}'", &command_path, &path);

                    let child = Command::new(command_path)
                        .current_dir(path)
                        .args(&start_info.args)
                        .spawn()
                        .expect("Can't start process");

                    let child_id = child.id();
                    started_process = Some(child);

                    println!("Done. ID={}", child_id);
                },
                WorkerCommand::Stop => {
                    if let Some(mut child) = mem::replace(&mut started_process, None) {
                        print!("Stopping process... ");
                        
                        child.kill().unwrap_or_else(|e| eprintln!("Error KILL process: {:?}", e));

                        println!("Done")
                    }
                }
                WorkerCommand::Update(update_version) => {
                    download_update(update_version, download_url.clone());
                }
            }
        }

        println!("Process stopped");
    })
}

fn download_update(update_version: u32, addr: Url) {
    print!("Downloading update {}... ", update_version);

    let file_name = format!("./data/download/update{}.zip", update_version);
    let mut file = std::fs::File::create(&file_name).unwrap();
    let _bytes_read = reqwest::get(addr)
        .unwrap()
        .copy_to(&mut file)
        .unwrap();

    println!("Done");

    print!("Extracting {:?}...", &file_name);

    unarchive_data(&file_name, "./data/unpacked").unwrap();

    println!("Done");
}

fn unarchive_data(path: &str, out_path: &str) -> io::Result<()> {
    let file = fs::File::open(path)?;
    let mut archive = ZipArchive::new(file)?;

    for index in 0..archive.len() {
        let mut zip_file = archive.by_index(index)?;

        let mut path = PathBuf::from(out_path);
        path.push(zip_file.sanitized_name());

        // println!("{:?}", &path);

        if zip_file.name().chars().rev().next().map_or(false, |c| c == '/' || c == '\\') {
            fs::create_dir_all(path).unwrap();
        } else {
            if let Some(parent_dir) = path.parent() {
                fs::create_dir_all(parent_dir).unwrap();
            }

            let mut out_file = fs::File::create(path).unwrap();
            io::copy(&mut zip_file, &mut out_file).unwrap();
        }
    }

    Ok(())
}