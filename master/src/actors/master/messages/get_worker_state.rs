use {
    ::actix::prelude::*,
    ::uuid::Uuid,
    crate::actors::{
        Result,
        Error,
        master::MasterActor,
        worker::messages::StatusMessage as WorkerStatusMessage
    }
};

pub struct GetWorkerState {
    pub worker_uid: String
}

pub struct GetWorkerStateResult {

}

impl Message for GetWorkerState {
    type Result = Result<GetWorkerStateResult>;
}

type GetWorkerStateResultFuture = Box<dyn Future<Item = GetWorkerStateResult, Error = Error>>;

impl Handler<GetWorkerState> for MasterActor {
    type Result = GetWorkerStateResultFuture;

    fn handle(&mut self, msg: GetWorkerState, _ctx: &mut Self::Context) -> Self::Result {
        let worker_uid = Uuid::parse_str(msg.worker_uid.as_str()).unwrap();
        let worker = self.get_worker_addr_by_uid(&worker_uid).unwrap();

        let worker_message = worker.send(WorkerStatusMessage {})
            .from_err()
            .and_then(|res| {
                match res {
                    Ok(_response) => {
                        Ok(GetWorkerStateResult {})
                    }
                    Err(e) => Err(e)
                }
            });

        Box::new(worker_message)
    }
}