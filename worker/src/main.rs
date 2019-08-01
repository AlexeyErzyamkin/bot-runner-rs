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
use std::process::Command;
use std::mem;

use reqwest;
use reqwest::Url;

use serde::Deserialize;

use shared;
use shared::models::{
    WorkerInfo, WorkerAction, StartInfo
};
use shared::{URL_SCOPE, URL_STATE, URL_UPDATE};
use shared::archiving;

const PATH_DATA: &str = "data/unpacked";
const PATH_DOWNLOAD: &str = "data/download";

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
    prepare_environment()?;

    let config_path = Path::new("./data/worker_config.json");
    let config: WorkerConfig = shared::read_config(config_path)?;

    let stop_flag = Arc::new(AtomicBool::default());
    let (tx, rx) = mpsc::channel();

    let scope_url = Url::parse(&config.addr).unwrap().join(URL_SCOPE).unwrap();
    let download_url = scope_url.join(URL_UPDATE).unwrap();
    let state_url = scope_url.join(URL_STATE).unwrap();

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

fn prepare_environment() -> io::Result<()> {
    fs::create_dir_all(PATH_DATA)?;
    fs::create_dir_all(PATH_DOWNLOAD)?;

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
                            WorkerAction::Start(ref start_info) => {
                                tx.send(WorkerCommand::Start(start_info.clone())).unwrap();
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
        let mut current_update_version = None;

        loop {
            let cmd = rx.recv().unwrap();
            match cmd {
                WorkerCommand::Quit => break,
                WorkerCommand::Start(start_info) => {
                    match current_update_version {
                        Some(update_version) if update_version == start_info.update_version => {
                            let mut path = PathBuf::from(PATH_DATA);
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
                        _ => eprintln!("Can't start process: Invalid update version")
                    }
                },
                WorkerCommand::Stop => {
                    if let Some(mut child) = mem::replace(&mut started_process, None) {
                        print!("Stopping process... ");
                        
                        child.kill().unwrap_or_else(|e| eprintln!("Error KILL process: {:?}", e));

                        println!("Done")
                    }
                }
                WorkerCommand::Update(update_version) => {
                    if let Ok(_) = download_update(update_version, download_url.clone()) {
                        current_update_version = Some(update_version);
                    }
                }
            }
        }

        println!("Process stopped");
    })
}

fn download_update(update_version: u32, addr: Url) -> io::Result<()> {
    print!("Downloading update {}... ", update_version);

    let file_name = format!("{}/{}.zip", PATH_DOWNLOAD, update_version);

    let mut file = std::fs::File::create(&file_name)?;

    match reqwest::get(addr) {
        Ok(ref mut response) if response.status().is_success() => {
            response.copy_to(&mut file).expect("Can't write to file")
        },
        _ => return Err(io::ErrorKind::Other.into())
    };

    println!("Done");

    print!("Extracting {:?}...", &file_name);

    archiving::unarchive_data(&file_name, PATH_DATA)?;

    println!("Done");

    Ok(())
}