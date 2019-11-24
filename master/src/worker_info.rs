use std::time::Instant;

pub enum WorkerState {
    Idle,
    Active
}

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct WorkerAddr(pub String);

pub struct WorkerInfo {
    pub state: WorkerState,
    pub last_state_time: Instant
}

impl WorkerInfo {
    pub fn new() -> Self {
        Self {
            state: WorkerState::Idle,
            last_state_time: Instant::now()
        }
    }
}