pub enum WorkerState {
    Idle,
    Active
}

pub struct WorkerInfo {
    pub state: WorkerState
}

impl WorkerInfo {
    pub fn new() -> Self {
        Self {
            state: WorkerState::Idle
        }
    }
}