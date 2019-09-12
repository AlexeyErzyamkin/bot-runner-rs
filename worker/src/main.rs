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
use shared::models::{ WorkerInfo, WorkerAction, UpdateVersion };
use shared::{ URL_SCOPE, URL_STATE, URL_UPDATE };
use shared::archiving;

use worker::WorkerCommand;

const PATH_DATA: &str = "data/unpacked";
const PATH_DOWNLOAD: &str = "data/download";

#[derive(Deserialize)]
struct WorkerConfig {
    addr: String
}

fn main() -> io::Result<()> {
    prepare_environment()?;

    let config_path = Path::new("./data/worker_config.json");
    let config: WorkerConfig = shared::read_config(config_path)?;

    let stop_flag = Arc::new(AtomicBool::default());
    let (tx, rx) = mpsc::channel();

    let base_url = Url::parse(&config.addr).unwrap();
    let download_url = base_url.join(&format!("{}{}", URL_SCOPE, URL_UPDATE)).unwrap();
    let state_url = base_url.join(&format!("{}{}", URL_SCOPE, URL_STATE)).unwrap();

    println!("State URL: {:?}", state_url);
    println!("State URL: {:?}", download_url);

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
                Ok(ref response) => {
                    eprintln!("Response status: {:?}", response.status());
                },
                Err(e) => {
                    eprintln!("Error updating status: {:?}", e);
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
                    let mut path = PathBuf::from(PATH_DATA);
                    if start_info.current_dir.len() > 0 {
                        path.push(start_info.current_dir);
                    }

                    let path = path;
                    // let path = path.canonicalize().unwrap();

                    let (is_relative, file_name) = {
                        if &start_info.command[..2] == "./" {
                            (true, &start_info.command[2..])
                        } else if &start_info.command[..3] == ".\\" {
                            (true, &start_info.command[3..])
                        } else {
                            (false, &start_info.command[..])
                        }
                    };

                    let command_path = if is_relative {
                        let mut command_path = PathBuf::from(&path);
                        command_path.push(file_name);

                        command_path
                    } else {
                        PathBuf::from(file_name)
                    };

                    println!("Starting command '{:?}' from '{:?}'", &command_path, &path);

                    let spawn_result = Command::new(command_path)
                        .current_dir(path)
                        .args(&start_info.args)
                        .spawn();

                    match spawn_result {
                        Ok(child) => {
                            let child_id = child.id();
                            started_process = Some(child);

                            println!("Done. ID={}", child_id);
                        },
                        Err(e) => {
                            eprintln!("Error spawning process: {:?}", e);
                        }
                    }
                },
                WorkerCommand::Stop => {
                    if let Some(mut child) = mem::replace(&mut started_process, None) {
                        println!("Stopping process... ");
                        
                        child.kill().unwrap_or_else(|e| eprintln!("Error KILL process: {:?}", e));

                        println!("Done");
                    }
                }
                WorkerCommand::Update(update_version) => {
                    loop {
                        match download_update(update_version, download_url.clone()) {
                            Ok(()) => break,
                            Err(e) => {
                                eprintln!("Error download update: {:?}", e);
                                
                                thread::sleep(std::time::Duration::from_secs(5));
                            }
                        }
                    }
                }
            }
        }

        println!("Process stopped");
    })
}

fn download_update(update_version: UpdateVersion, addr: Url) -> io::Result<()> {
    println!("Downloading update {}... ", update_version.0);

    let file_name = format!("{}/{}.zip", PATH_DOWNLOAD, update_version.0);

    let mut file = std::fs::File::create(&file_name)?;

    match reqwest::get(addr) {
        Ok(ref mut response) if response.status().is_success() => {
            response.copy_to(&mut file).expect("Can't write to file")
        },
        _ => return Err(io::ErrorKind::Other.into())
    };

    println!("Done");

    println!("Extracting {:?}...", &file_name);

    archiving::unarchive_data(&file_name, PATH_DATA)?;

    println!("Done");

    Ok(())
}