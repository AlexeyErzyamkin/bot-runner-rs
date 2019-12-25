use {
    ::actix::prelude::*,
    shared::models::StartInfo,
    crate::actors::{
        Result,
        Error,
        master::MasterActor
    }
};

pub struct StartJob {
    pub workers_count: i32,
    pub start_info: StartInfo
}

impl Message for StartJob {
    type Result = Result<(), MailboxError>;
}

// type StartJobResultFuture = Box<dyn Future<Output = Result<(), Error>>>;
// type StartJobResultFuture = ResponseActFuture<Self, Result<(), Error>>;

impl Handler<StartJob> for MasterActor {
    type Result = ResponseActFuture<Self, Result<(), MailboxError>>;

    fn handle(&mut self, msg: StartJob, ctx: &mut Self::Context) -> Self::Result {
        // for worker in self.

        unimplemented!();
    }
}