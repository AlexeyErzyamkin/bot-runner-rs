use std::default::Default;
use std::fmt::{Display, Formatter};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct UpdateVersion(u32);

impl UpdateVersion {
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl Display for UpdateVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct StateVersion(u32);

impl StateVersion {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

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

impl WorkerInfo {
    pub fn new(version: StateVersion, update_version: UpdateVersion, action: WorkerAction) -> Self {
        Self {
            version,
            update_version,
            action
        }
    }
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StartInfo {
    pub command: String,
    pub current_dir: String,
    pub args: Vec<String>,
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