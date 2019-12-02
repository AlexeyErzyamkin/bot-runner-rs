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
    }
};

pub struct RegisterWorker {

}

pub struct RegisterWorkerResult {
    pub id: String
}

impl Message for RegisterWorker {
    type Result = Result<RegisterWorkerResult>;
}

impl Handler<RegisterWorker> for MasterActor {
    type Result = <RegisterWorker as Message>::Result;

    fn handle(&mut self, _msg: RegisterWorker, _ctx: &mut Self::Context) -> Self::Result {
        let worker_uid = uuid::Uuid::new_v4();
        let worker_uid_str = format!("{}", &worker_uid);
        let worker_id = next_id(&mut self.last_worker_id);

        let addr = WorkerActor::new(worker_id).start();

        self.add_worker(worker_uid, worker_id, addr);

        Ok(RegisterWorkerResult { id: worker_uid_str })
    }
}

fn next_id(id: &mut WorkerId) -> WorkerId {
    id.0 += 1;
    WorkerId(id.0)
}