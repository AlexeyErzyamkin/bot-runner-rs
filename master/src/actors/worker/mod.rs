pub mod messages;

use {
    ::std::time::{
        Instant,
        Duration
    },
    ::actix::prelude::*,
    crate::actors::master::{
        MasterAddr,
        messages::WorkerDead
    }
};

const WORKER_DIE_TIMEOUT: Duration = Duration::from_secs(60);

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct WorkerId(pub u32);

pub enum WorkerStatus {
    Idle
}

pub struct WorkerActor {
    master: MasterAddr,
    state: WorkerState
}

impl WorkerActor {
    pub fn new(id: WorkerId, master: MasterAddr) -> Self {
        Self {
            master,
            state: WorkerState::new(id)
        }
    }

    fn schedule_try_die(&mut self, ctx: &mut Context<Self>) {
        ctx.run_later(
            Duration::from_secs(30),
            |a, ac| {
                if !a.try_die(ac) {
                    a.schedule_try_die(ac);
                }
            }
        );
    }

    fn try_die(&mut self, ctx: &mut Context<Self>) -> bool {
        let time_to_die = self.state.last_update_time.elapsed() > WORKER_DIE_TIMEOUT;
        if time_to_die {
            self.master.do_send(WorkerDead {
                id: self.state.id
            });

            ctx.stop();
        }

        time_to_die
    }
}

pub type WorkerAddr = Addr<WorkerActor>;

impl Actor for WorkerActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.schedule_try_die(ctx);
    }
}

struct WorkerState {
    id: WorkerId,
    last_update_time: Instant,
    status: WorkerStatus
//    die_timer_handle: SpawnHandle
}

impl WorkerState {
    pub(crate) fn new(id: WorkerId) -> Self {
        Self {
            id,
            last_update_time: Instant::now(),
            status: WorkerStatus::Idle
        }
    }

    pub(crate) fn update(&mut self) {
        self.last_update_time = Instant::now();
    }

    pub(crate) fn set_status(&mut self, status: WorkerStatus) {
        self.status = status;
    }
}