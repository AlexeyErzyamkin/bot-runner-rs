use {
    ::actix::prelude::*,
    ::uuid::Uuid,
    crate::actors::{
        Result,
        Error,
        master::MasterActor,
        worker::messages::StatusMessage as WorkerStatusMessage
    },
    shared::models::WorkerKey
};

pub struct GetWorkerState {
    pub worker_key: WorkerKey
}

pub struct GetWorkerStateResponse {}

pub type GetWorkerStateResult = Result<GetWorkerStateResponse>;

impl Message for GetWorkerState {
    type Result = GetWorkerStateResult;
}

impl Handler<GetWorkerState> for MasterActor {
    type Result = ResponseActFuture<Self, GetWorkerStateResult>;

    fn handle(&mut self, msg: GetWorkerState, _ctx: &mut Self::Context) -> Self::Result {
        let worker_uid = Uuid::parse_str(msg.worker_key.0.as_str()).unwrap();
        let worker = self.get_worker_addr_by_uid(&worker_uid).unwrap();

        let worker_message = worker
            .send(WorkerStatusMessage {})
            .into_actor(self)
            .map(|send_res, a, _| {
                match send_res {
                    Ok(res) => {
                        match res {
                            Ok(response) => Ok(GetWorkerStateResponse {}),
                            Err(e) => Err(e)
                        }
                    }
                    Err(e) => Err(Error::InternalError)
                }
            });

        Box::new(worker_message)
    }
}