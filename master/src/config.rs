use serde::Deserialize;

use std::collections::HashMap;
use std::io;
use std::path::Path;

use shared::models::{
    StartInfo
};

#[derive(Deserialize)]
pub struct MasterConfig {
    pub addr: String,
    pub data_path: String,
    pub start_infos: HashMap<String, StartInfo>
}

pub fn read(path: &str) -> io::Result<MasterConfig> {
    let config_path = Path::new(path);
    let config: MasterConfig = shared::read_config(config_path)?;

    if config.start_infos.is_empty() {
        eprintln!("Start infos collection is empty");

        return Err(io::ErrorKind::InvalidData.into());
    }

    Ok(config)
}