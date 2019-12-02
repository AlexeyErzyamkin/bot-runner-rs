use {
    ::actix::prelude::*,
    crate::{
        actors::{
            Result,
            Error,
            worker::WorkerId,
            master::MasterActor
        }
    }
};

pub struct WorkerDead {
    pub id: WorkerId
}

impl Message for WorkerDead {
    type Result = Result<()>;
}

impl Handler<WorkerDead> for MasterActor {
    type Result = Result<()>; //Box<dyn Future<Item = (), Error = Error>>;

    fn handle(&mut self, msg: WorkerDead, ctx: &mut Self::Context) -> Self::Result {
        unimplemented!()
    }
}