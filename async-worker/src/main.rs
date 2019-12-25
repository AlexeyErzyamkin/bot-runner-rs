mod process;
mod update_status;

use {
    std::{
        path::{
            Path
        }
    },
    ::tokio::{
        fs,
        sync::mpsc::{
            self,
            Sender,
            Receiver
        }
    },
    ::serde::Deserialize,
    ::surf::{
        url::Url
    },
    ::futures::{
        try_join
    },
    shared::{
        URL_SCOPE, URL_STATE, URL_UPDATE,
        models::{
            UpdateVersion, StartInfo,
            RegisterRequest, RegisterResponse,
            WorkerKey
        }
    },
    process::process,
    update_status::update_status
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

type Tx = Sender<WorkerCommand>;
type Rx = Receiver<WorkerCommand>;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    prepare_environment().await?;

    let config_path = Path::new("./worker_config.json");
    let config: WorkerConfig = shared::read_config(config_path)?;

    let base_url = Url::parse(&config.addr).unwrap();
    let download_url = base_url.join(&format!("{}{}", URL_SCOPE, URL_UPDATE)).unwrap();
    let state_url = base_url.join(&format!("{}{}", URL_SCOPE, URL_STATE)).unwrap();
    let register_url = base_url.join("v2/register").unwrap();

    let (tx, rx) = mpsc::channel(10);

    let process_handle = tokio::spawn(async move {
        process(rx, download_url).await
    });

    let update_status_handle = tokio::spawn(async move {
        update_status(tx, state_url).await
    });

    match try_join!(process_handle, update_status_handle) {
        Ok((a, b)) => a.and(b),
        Err(e) => {
            eprintln!("Error: {:?}", e);

            Err(std::io::ErrorKind::Other.into())
        }
    }
}

async fn prepare_environment() -> std::io::Result<()> {
    fs::create_dir_all(PATH_DATA).await?;
    fs::create_dir_all(PATH_DOWNLOAD).await?;

    Ok(())
}

async fn register(url: Url) -> Result<WorkerKey, String> {
    match surf::post(&url).recv_json::<RegisterResponse>().await {
        Ok(response) => Ok(response.key),
        Err(_) => Err("".to_string())
    }
}