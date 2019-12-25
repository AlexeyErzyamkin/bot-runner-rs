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

//Box<dyn Future<Item = (), Error = Error>>;

impl Handler<WorkerDead> for MasterActor {
    type Result = Result<()>;

    fn handle(&mut self, msg: WorkerDead, _ctx: &mut Self::Context) -> Self::Result {
        self.remove_worker(msg.id);

        Ok(())
    }
}