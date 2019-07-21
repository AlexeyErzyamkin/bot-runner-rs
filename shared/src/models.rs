use std::default::Default;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum WorkerAction {
    Start,
    Stop,
    Update
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkerInfo {
    pub version: u32,
    pub action: WorkerAction,
    pub update_url: String,
    pub start_command_line: String
}

impl Default for WorkerInfo {
    fn default() -> Self {
        Self {
            version: 0,
            action: WorkerAction::Stop,
            update_url: String::new(),
            start_command_line: String::new()
        }
    }
}