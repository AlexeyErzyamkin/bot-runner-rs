use {
    std::{
        self,
        mem,
        path::{PathBuf}
    },
    ::tokio::{
        time, fs,
        sync::mpsc::Receiver,
        process::Command
    },
    ::surf::{
        self,
        url::Url
    },
    shared::{
        self,
        models::{
            UpdateVersion
        },
        archiving
    },
    crate::{
        WorkerCommand,
        Rx
    }
};

const PATH_DATA: &str = "data/unpacked";
const PATH_DOWNLOAD: &str = "data/download";

pub async fn process(mut rx: Rx, download_url: Url) -> std::io::Result<()> {
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
                        .kill_on_drop(true)
                        .spawn();

                    match spawn_result {
                        Ok(child) => {
                            let child_id = child.id();
                            started_process = Some(child);

                            println!("Done. ID={}", child_id);
                            // println!("Done");
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
                        match download_update(update_version, download_url.clone()).await {
                            Ok(()) => break,
                            Err(e) => {
                                eprintln!("Error download update: {:?}", e);
                                
                                time::delay_for(std::time::Duration::from_secs(5)).await;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Process stopped");

    Ok(())
}

async fn download_update(update_version: UpdateVersion, addr: Url) -> std::io::Result<()> {
    println!("Downloading update {}... ", update_version.0);

    let file_name = format!("{}/{}.zip", PATH_DOWNLOAD, update_version.0);

    let mut file = fs::File::open(&file_name).await?;

    match surf::get(addr).await {
        Ok(ref mut response) if response.status().is_success() => {
            let bytes = response.body_bytes().await?;
            ::tokio::io::copy(&mut &bytes[..], &mut file).await?;
        }
        _ => return Err(std::io::ErrorKind::Other.into())
    };

    println!("Done");

    println!("Extracting {:?}...", &file_name);

    archiving::unarchive_data(&file_name, PATH_DATA)?;

    println!("Done");

    Ok(())
}