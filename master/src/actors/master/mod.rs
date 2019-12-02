pub mod messages;

use {
    std::{
        collections::HashMap,
        time::Duration
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

type WorkerAddr = Addr<WorkerActor>;

pub struct MasterActor {
    last_worker_id: WorkerId,
    worker_ids_by_uid: HashMap<Uuid, WorkerId>,
    workers_by_id: HashMap<WorkerId, WorkerAddr>
}

impl MasterActor {
    pub fn new() -> Self {
        let actor = Self {
            last_worker_id: WorkerId(0),
            worker_ids_by_uid: HashMap::new(),
            workers_by_id: HashMap::new()
        };

        actor
    }

    pub(crate) fn get_worker_addr_by_uid(&self, uid: &Uuid) -> Option<&WorkerAddr> {
        match self.worker_ids_by_uid.get(uid) {
            Some(wid) => self.workers_by_id.get(wid),
            None => None
        }
    }

    pub(crate) fn get_worker_addr_by_id(&self, id: &WorkerId) -> Option<&WorkerAddr> {
        self.workers_by_id.get(id)
    }
}

impl Actor for MasterActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
//        let drop_dead_workers_handle = ctx.run_interval(
//            Duration::from_secs(60),
//            |actor, _actor_ctx| actor.drop_dead_workers()
//        );
    }
}
