use std::default::Default;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct UpdateVersion(pub u32);

// impl UpdateVersion {
//     pub fn next(&self) -> Self {
//         Self(self.0 + 1)
//     }
// }

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct StateVersion(pub u32);

// impl StateVersion {
//     pub fn next(&self) -> Self {
//         Self(self.0 + 1)
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
pub enum WorkerAction {
    Start(StartInfo),
    Stop,
    Update
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkerInfo {
    pub version: StateVersion,
    pub update_version: UpdateVersion,
    pub action: WorkerAction
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
            version: StateVersion::default(),
            update_version: UpdateVersion::default(),
            action: WorkerAction::Stop
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