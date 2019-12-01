pub mod messages;

use {
    std::{
        collections::HashMap
    },
    actix::prelude::*,
    uuid::Uuid,
    crate::actors::{
        worker::{
            WorkerId,
            WorkerActor
        },
    }
};

pub enum ResultError {
    Unknown
}

pub struct MasterActor {
    last_worker_id: WorkerId,
    pub workers: HashMap<Uuid, Addr<WorkerActor>>
}

impl MasterActor {
    pub fn new() -> Self {
        Self {
            last_worker_id: WorkerId(0),
            workers: HashMap::new()
        }
    }
}

impl Actor for MasterActor {
    type Context = Context<Self>;
}
