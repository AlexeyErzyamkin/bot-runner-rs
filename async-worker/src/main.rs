mod process;

use {
    std::{
        io, fs,
        path::{
            Path, PathBuf
        },
        time::Duration
    },
    ::tokio::{
        time,
        sync::mpsc::{
            self,
            Sender, Receiver
        }
    },
    ::serde::Deserialize,
    ::surf::{
        self, url::Url
    },
    shared::{
        URL_SCOPE, URL_STATE, URL_UPDATE,
        models::{
            WorkerInfo, WorkerAction, UpdateVersion, StartInfo
        }
    }
};

const PATH_DATA: &str = "data/unpacked";
const PATH_DOWNLOAD: &str = "data/download";

#[derive(Deserialize)]
struct WorkerConfig {
    addr: String
}

#[derive(Debug)]
pub enum WorkerCommand {
    Stop,
    Start(StartInfo),
    Update(UpdateVersion),
    Quit
}

#[tokio::main]
async fn main() -> io::Result<()> {
    prepare_environment()?;

    let config_path = Path::new("./worker_config.json");
    let config: WorkerConfig = shared::read_config(config_path)?;

    let base_url = Url::parse(&config.addr).unwrap();
    let download_url = base_url.join(&format!("{}{}", URL_SCOPE, URL_UPDATE)).unwrap();
    let state_url = base_url.join(&format!("{}{}", URL_SCOPE, URL_STATE)).unwrap();

    let (mut tx, mut rx): (Sender<WorkerCommand>, Receiver<WorkerCommand>) = mpsc::channel(100);

    let mut prev_status = WorkerInfo::default();

    loop {
        match surf::get(&state_url).recv_json::<WorkerInfo>().await {
            Ok(status) => {
                if status.version > prev_status.version {
                    tx.send(WorkerCommand::Stop).await.unwrap();

                    if status.update_version > prev_status.update_version {
                        tx.send(WorkerCommand::Update(status.update_version)).await.unwrap();
                    }

                    match status.action {
                        WorkerAction::Start(ref start_info) => {
                            tx.send(WorkerCommand::Start(start_info.clone())).await.unwrap();
                        },
                        WorkerAction::Stop => (),
                        WorkerAction::Update => ()
                    };
                }

                prev_status = status;
            }
            Err(e) => {

            }
        }

        time::delay_for(Duration::from_secs(10)).await;
    }

//    Ok(())
}

fn prepare_environment() -> io::Result<()> {
    fs::create_dir_all(PATH_DATA)?;
    fs::create_dir_all(PATH_DOWNLOAD)?;

    Ok(())
}