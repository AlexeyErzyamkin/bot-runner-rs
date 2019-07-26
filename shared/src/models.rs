use std::default::Default;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum WorkerAction {
    Start(StartInfo),
    Stop,
    Update
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkerInfo {
    pub version: u32,
    pub update_version: u32,
    pub action: WorkerAction,
    // pub start_info: Option<StartInfo>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StartInfo {
    pub command: String,
    pub current_dir: String,
    pub args: Vec<String>,
}

impl Default for WorkerInfo {
    fn default() -> Self {
        Self {
            version: 0,
            update_version: 0,
            action: WorkerAction::Stop,
            // start_info: None
        }
    }
}

impl Default for StartInfo {
    fn default() -> Self {
        Self {
            command: String::new(),
            current_dir: String::new(),
            args: Vec::new()
        }
    }
}