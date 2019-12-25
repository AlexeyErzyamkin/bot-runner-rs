use std::default::Default;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct UpdateVersion(pub u32);

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct StateVersion(pub u32);

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

#[derive(Serialize, Deserialize, Clone)]
pub struct WorkerKey(pub String);

// impl WorkerKey {
//    pub fn new(value: String) -> Self {
//        Self(S)
//    }

//    pub fn to_next(&mut self) -> Self {
//        self.0 += 1;

//        Self(self.0)
//    }
// }

#[derive(Serialize, Deserialize)]
pub struct EmptyResponse;

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {

}

#[derive(Serialize, Deserialize)]
pub struct RegisterResponse {
    pub key: WorkerKey
}

#[derive(Serialize, Deserialize)]
pub struct StateRequest {
    pub key: WorkerKey
}

#[derive(Serialize, Deserialize)]
pub struct StateResponse {

}

// #[derive(Serialize, Deserialize)]
// pub struct StartJobRequest {
    
// }

//#[derive(Serialize, Deserialize)]
//pub struct ErrorResponse {
//    pub error: String
//}