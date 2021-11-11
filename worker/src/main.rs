use std::mem;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time;

use tokio::{
    fs,
    sync::mpsc::{self, Receiver, Sender},
};

use reqwest::{self, Client, Url};

use serde::Deserialize;

use shared;
use shared::archiving;
use shared::models::{UpdateVersion, WorkerAction, WorkerInfo};
use shared::{URL_SCOPE, URL_STATE, URL_UPDATE};

use worker::WorkerCommand;
use worker::{Error, Result};

use futures::future;

const PATH_DATA: &str = "data/unpacked";
const PATH_DOWNLOAD: &str = "data/download";

#[derive(Deserialize)]
struct WorkerConfig {
    addr: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    prepare_environment().await?;

    let config_path = Path::new("./data/worker_config.json");
    let config: WorkerConfig = shared::read_config(config_path)?;

    let stop_flag = Arc::new(AtomicBool::default());
    let (tx, rx) = mpsc::channel(10);

    let base_url = Url::parse(&config.addr)?;
    let download_url = base_url.join(&format!("{}{}", URL_SCOPE, URL_UPDATE))?;
    let state_url = base_url.join(&format!("{}{}", URL_SCOPE, URL_STATE))?;

    println!("State URL: {:?}", state_url);
    println!("State URL: {:?}", download_url);

    let client = Client::new();

    let us_thread_handle = update_status(&client, stop_flag.clone(), tx, state_url);
    let p_thread_handle = process(&client, rx, download_url);

    let _result = future::join(us_thread_handle, p_thread_handle).await;

    //    println!("Press ENTER to exit...");
    //
    //    let mut buf = String::new();
    //    std::io::stdin()
    //        .read_line(&mut buf)
    //        .expect("Read input failed");
    //
    //    stop_flag.store(true, Ordering::Relaxed);

    Ok(())
}

async fn prepare_environment() -> Result<()> {
    fs::create_dir_all(PATH_DATA).await?;
    fs::create_dir_all(PATH_DOWNLOAD).await?;

    Ok(())
}

async fn update_status(
    client: &Client,
    stop_flag: Arc<AtomicBool>,
    tx: Sender<WorkerCommand>,
    addr: Url,
) -> Result<()> {
    let mut prev_status = WorkerInfo::default();

    loop {
        if stop_flag.load(Ordering::Relaxed) {
            tx.send(WorkerCommand::Quit).await?;
            break;
        }

        match client.get(addr.clone()).send().await {
            Ok(response) if response.status().is_success() => {
                let status = response.json::<WorkerInfo>().await?;

                if status.version > prev_status.version {
                    tx.send(WorkerCommand::Stop).await?;

                    if status.update_version > prev_status.update_version {
                        tx.send(WorkerCommand::Update(status.update_version))
                            .await?;
                    }

                    match status.action {
                        WorkerAction::Start(ref start_info) => {
                            tx.send(WorkerCommand::Start(start_info.clone())).await?;
                        }
                        WorkerAction::Stop => (),
                        WorkerAction::Update => (),
                    };
                }

                prev_status = status;
            }
            Ok(ref response) => {
                eprintln!("Response status: {:?}", response.status());
            }
            Err(e) => {
                eprintln!("Error updating status: {:?}", e);
            }
        }

        tokio::time::sleep(time::Duration::from_secs(10)).await;
    }

    Ok(())
}

async fn process(
    client: &Client,
    mut rx: Receiver<WorkerCommand>,
    download_url: Url,
) -> Result<()> {
    let mut started_process = None;

    loop {
        if let Some(cmd) = rx.recv().await {
            match cmd {
                WorkerCommand::Quit => break,
                WorkerCommand::Start(start_info) => {
                    let mut path = PathBuf::from(PATH_DATA);
                    if start_info.current_dir.len() > 0 {
                        path.push(start_info.current_dir);
                    }

                    let path = path;

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
                        }
                        Err(e) => {
                            eprintln!("Error spawning process: {:?}", e);
                        }
                    }
                }
                WorkerCommand::Stop => {
                    if let Some(mut child) = mem::replace(&mut started_process, None) {
                        println!("Stopping process... ");

                        child
                            .kill()
                            .unwrap_or_else(|e| eprintln!("Error KILL process: {:?}", e));

                        println!("Done");
                    }
                }
                WorkerCommand::Update(update_version) => loop {
                    match download_update(client, update_version, download_url.clone()).await {
                        Ok(()) => break,
                        Err(e) => {
                            eprintln!("Error download update: {:?}", e);

                            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                        }
                    }
                },
            }
        }
    }

    println!("Process stopped");

    Ok(())
}

async fn download_update(client: &Client, update_version: UpdateVersion, addr: Url) -> Result<()> {
    println!("Downloading update {}... ", update_version);

    let file_name = format!("{}/{}.zip", PATH_DOWNLOAD, update_version);

    let response = client.get(addr).send().await?;

    if response.status().is_success() {
        let bytes = response.bytes().await?;
        fs::write(&file_name, bytes).await?;
    } else {
        return Err(Error::Server);
    }

    println!("Done");

    println!("Extracting {:?}...", &file_name);

    archiving::unarchive_data(&file_name, PATH_DATA)?;

    println!("Done");

    Ok(())
}
