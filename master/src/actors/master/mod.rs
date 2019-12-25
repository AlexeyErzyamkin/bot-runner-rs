pub mod messages;

use {
    std::{
        collections::HashMap,
        mem
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

#[derive(PartialEq, Debug, Copy, Clone)]
struct WorkerInfoVersion(i32);

struct WorkerInfoHolder {
    pub version: WorkerInfoVersion,
    pub info: Box<WorkerInfo>
}

pub struct WorkerInfo {
    pub id: WorkerId,
    pub uid: Uuid,
    pub addr: WorkerAddr
}

#[derive(Copy, Clone)]
struct WorkerInfoIndex {
    pub version: WorkerInfoVersion,
    pub index: usize
}

pub struct MasterActor {
    last_worker_id: WorkerId,
    last_woker_info_version: WorkerInfoVersion,
    workers: Vec<Option<WorkerInfoHolder>>,
    worker_ids_by_uid: HashMap<Uuid, WorkerInfoIndex>,
    workers_by_id: HashMap<WorkerId, WorkerInfoIndex>
}

impl MasterActor {
    pub fn new() -> Self {
        let actor = Self {
            last_worker_id: WorkerId(0),
            last_woker_info_version: WorkerInfoVersion(0),
            workers: Vec::new(),
            worker_ids_by_uid: HashMap::new(),
            workers_by_id: HashMap::new()
        };

        actor
    }

    pub(crate) fn get_worker_addr_by_uid(&self, uid: &Uuid) -> Option<&WorkerAddr> {
        match self.worker_ids_by_uid.get(uid) {
            Some(ref wi) => match self.get_worker(wi) {
                Some(ref winfo) => Some(&winfo.addr),
                None => None
            }
            None => None
        }
    }

    pub(crate) fn get_worker_addr_by_id(&self, id: &WorkerId) -> Option<&WorkerAddr> {
        match self.workers_by_id.get(id) {
            Some(ref wi) => match self.get_worker(wi) {
                Some(ref winfo) => Some(&winfo.addr),
                None => None
            }
            None => None
        }
    }

    pub(crate) fn add_worker(&mut self, uid: Uuid, id: WorkerId, addr: WorkerAddr) {
        self.last_woker_info_version.0 += 1;

        let new_holder = Some(WorkerInfoHolder {
            version: self.last_woker_info_version,
            info: Box::new(
                WorkerInfo {
                    id,
                    uid,
                    addr
                }
            )
        });

        for (i, holder) in self.workers.iter_mut().enumerate() {
            match holder {
                None => {
                    mem::replace(holder, new_holder);
                    
                    let index = WorkerInfoIndex {
                        version: self.last_woker_info_version,
                        index: i
                    };

                    add_indexes(self, uid, id, index);

                    return;
                }
                _ => {}
            }
        }

        self.workers.push(new_holder);
        let index = WorkerInfoIndex {
            version: self.last_woker_info_version,
            index: self.workers.len() - 1
        };

        add_indexes(self, uid, id, index);

        fn add_indexes(me: &mut MasterActor, uid: Uuid, id: WorkerId, index: WorkerInfoIndex) {
            me.worker_ids_by_uid.insert(uid, index);
            me.workers_by_id.insert(id, index);
        }
    }

    pub (crate) fn remove_worker(&mut self, id: WorkerId) {
        match self.workers_by_id.remove(&id) {
            Some(wi) => {
                if let Some(wih) = self.workers.remove(wi.index) {
                    assert_eq!(wi.version, wih.version);

                    self.worker_ids_by_uid.remove(&(*wih.info).uid);
                }
            }
            None => panic!("Unknown worker id")
        }
    }

    fn get_worker(&self, index: &WorkerInfoIndex) -> Option<&WorkerInfo> {
        if let Some(ref ih) = self.workers[index.index] {
            if ih.version == index.version {
                return Some(&ih.info)
            }
        }

        None
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
