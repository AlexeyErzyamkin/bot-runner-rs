use {
    ::actix::prelude::*,
    crate::{
        actors::{
            Result,
            worker::WorkerActor
        }
    }
};

pub struct StatusMessage {}

pub struct StatusResponse {}

pub type StatusMessageResult = Result<StatusResponse>;

impl Message for StatusMessage {
    type Result = StatusMessageResult;
}

impl Handler<StatusMessage> for WorkerActor {
    type Result = StatusMessageResult;

    fn handle(&mut self, _msg: StatusMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.state.update();

        Ok(StatusResponse {})
    }
}