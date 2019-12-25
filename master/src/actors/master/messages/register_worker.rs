use {
    ::actix::prelude::*,
    crate::actors::{
        Result,
        master::{
            MasterActor
        },
        worker::{
            WorkerId,
            WorkerActor
        }
    },
    shared::models::WorkerKey
};

pub struct RegisterWorker {

}

pub struct RegisterWorkerResult {
    pub key: WorkerKey
}

impl Message for RegisterWorker {
    type Result = Result<RegisterWorkerResult>;
}

impl Handler<RegisterWorker> for MasterActor {
    type Result = <RegisterWorker as Message>::Result;

    fn handle(&mut self, _msg: RegisterWorker, ctx: &mut Self::Context) -> Self::Result {
        let worker_uid = uuid::Uuid::new_v4();
        let worker_key = WorkerKey(worker_uid.to_string()); //format!("{}", &worker_uid);
        let worker_id = next_id(&mut self.last_worker_id);

        let addr = WorkerActor::new(worker_id, ctx.address()).start();

        self.add_worker(worker_uid, worker_id, addr);

        Ok(RegisterWorkerResult { key: worker_key })
    }
}

fn next_id(id: &mut WorkerId) -> WorkerId {
    id.0 += 1;
    WorkerId(id.0)
}