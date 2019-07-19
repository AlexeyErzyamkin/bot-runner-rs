use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct WorkerInfo {
    pub version: u32,
    pub action: u8
}