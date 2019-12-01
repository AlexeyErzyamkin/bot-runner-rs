pub mod messages;

use {
    ::std::time::Instant,
    ::actix::prelude::*
};

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct WorkerId(pub u32);

pub enum WorkerStatus {
    Idle
}

pub struct WorkerActor {
    state: WorkerState
}

impl WorkerActor {
    pub fn new(id: WorkerId) -> Self {
        Self {
            state: WorkerState::new(id)
        }
    }
}

impl Actor for WorkerActor {
    type Context = Context<Self>;
}

struct WorkerState {
    id: WorkerId,
    last_update_time: Instant,
    status: WorkerStatus
}

impl WorkerState {
    pub(crate) fn new(id: WorkerId) -> Self {
        Self {
            id,
            last_update_time: Instant::now(),
            status: WorkerStatus::Idle
        }
    }

    pub(crate) fn update(&mut self) {
        self.last_update_time = Instant::now();
    }

    pub(crate) fn set_status(&mut self, status: WorkerStatus) {
        self.status = status;
    }
}