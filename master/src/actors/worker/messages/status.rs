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

impl Message for StatusMessage {
    type Result = Result<StatusResponse>;
}

impl Handler<StatusMessage> for WorkerActor {
    type Result = <StatusMessage as Message>::Result;

    fn handle(&mut self, _msg: StatusMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.state.update();

        Ok(StatusResponse {})
    }
}