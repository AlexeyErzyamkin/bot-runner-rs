use {
    std::{
        collections::HashMap
    },
    actix::prelude::*,
    super::{
        Result,
//        prelude::*,
        worker::{
            WorkerId,
            WorkerActor
        },
    }
};

pub struct MasterActor {
    last_worker_id: WorkerId,
    pub workers: HashMap<WorkerId, Addr<WorkerActor>>
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


pub struct RegisterWorker {

}

pub struct RegisterWorkerResult {
//    worker_id: WorkerId
    pub id: String
}

impl Message for RegisterWorker {
    type Result = Result<RegisterWorkerResult>;
}

//impl MessageResponse<MasterActor, RegisterWorker> for RegisterWorkerResult {
//
//}

impl Handler<RegisterWorker> for MasterActor {
    type Result = <RegisterWorker as Message>::Result;

    fn handle(&mut self, _msg: RegisterWorker, _ctx: &mut Self::Context) -> Self::Result {
        let worker_id = next_id(&mut self.last_worker_id);

        let addr = WorkerActor::create(move |_ctx| WorkerActor::new(worker_id.clone()));

        self.workers.insert(worker_id, addr);

        Ok(RegisterWorkerResult { id: "id".to_string() })
    }
}

fn next_id(id: &mut WorkerId) -> WorkerId {
    id.0 += 1;
    WorkerId(id.0)
}