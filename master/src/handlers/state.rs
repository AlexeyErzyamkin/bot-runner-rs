use std::sync::RwLock;

use actix_web::{
    Responder,
    web
};

use shared;
use shared::models::{
    WorkerInfo,
    WorkerAction
};
use crate::state::{State, Action};

pub fn handle_state(state: web::Data<RwLock<State>>) -> impl Responder {
    let state_read = state.read().unwrap();

    let worker_info = WorkerInfo {
        version: state_read.version,
        update_version: state_read.update_version,
        action: match state_read.action {
            Action::Update => WorkerAction::Update,
            Action::Start(ref start_info) => {
                let start_info = state_read.start_infos
                    .get(start_info)
                    .expect("Start info must be present");

                WorkerAction::Start(start_info.clone())
            },
            Action::Stop => WorkerAction::Stop
        }
    };

    web::Json(worker_info)
}