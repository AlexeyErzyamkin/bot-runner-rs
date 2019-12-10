use {
    std::{
        self,
        io, mem,
        path::{Path, PathBuf},
        sync::{
            Arc,
            atomic::{
                AtomicBool, Ordering
            }
        },
        future::Future
    },
    ::tokio::{
        time, fs,
        io::{
//            self,
            AsyncRead
        },
        sync::mpsc::Receiver as TokReceiver
    },
    ::surf::{
        self,
        url::Url
    },
    ::serde::Deserialize,
//    ::futures_io::AsyncRead,
    shared::{
        self,
        URL_SCOPE, URL_STATE, URL_UPDATE,
        models::{
            WorkerInfo, WorkerAction, UpdateVersion
        },
        archiving
    },
    crate::WorkerCommand
};

//use std::process::Command;

const PATH_DATA: &str = "data/unpacked";
const PATH_DOWNLOAD: &str = "data/download";

pub struct ProcessParams {

}

pub async fn process(rx: TokReceiver<WorkerCommand>, download_url: Url) -> std::io::Result<()> {
    Ok(())
}

async fn download_update(update_version: UpdateVersion, addr: Url) -> std::io::Result<()> {
    println!("Downloading update {}... ", update_version.0);

    let file_name = format!("{}/{}.zip", PATH_DOWNLOAD, update_version.0);

    let mut file = fs::File::open(&file_name).await?;

    match surf::get(addr).await {
        Ok(ref mut response) if response.status().is_success() => {
            let mut bytes = response.body_bytes().await?;
            ::tokio::io::copy(&mut &bytes[..], &mut file).await?;
//            ::tokio::io::copy(&mut &response, &mut file).await?;
        }
        _ => return Err(io::ErrorKind::Other.into())
    };

    println!("Done");

    println!("Extracting {:?}...", &file_name);

    archiving::unarchive_data(&file_name, PATH_DATA)?;

    println!("Done");

    Ok(())
}