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
            WorkerActor,
            WorkerAddr
        },
    }
};

//pub enum ResultError {
//    Unknown
//}

pub type MasterAddr = Addr<MasterActor>;

pub struct WorkerInfo {
    pub uid: Uuid,
    pub addr: WorkerAddr
}

pub struct MasterActor {
    last_worker_id: WorkerId,
    worker_ids_by_uid: HashMap<Uuid, WorkerId>,
    workers_by_id: HashMap<WorkerId, WorkerInfo>
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
            Some(wid) => self.get_worker_addr_by_id(wid),
            None => None
        }
    }

    pub(crate) fn get_worker_addr_by_id(&self, id: &WorkerId) -> Option<&WorkerAddr> {
        match self.workers_by_id.get(id) {
            Some(winfo) => Some(&winfo.addr),
            None => None
        }
    }

    pub(crate) fn add_worker(&mut self, uid: Uuid, id: WorkerId, addr: WorkerAddr) {
        self.worker_ids_by_uid.insert(uid, id);
        self.workers_by_id.insert(id, WorkerInfo {
            uid,
            addr
        });
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
