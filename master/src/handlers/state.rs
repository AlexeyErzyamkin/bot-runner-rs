use std::sync::RwLock;

use actix_web::{
    Responder,
    web,
    HttpRequest
};

use shared;
use shared::models::{
    WorkerInfo,
    WorkerAction
};
use crate::state::{State, Action};

pub async fn handle_state(_request: HttpRequest, state: web::Data<RwLock<State>>) -> impl Responder {
    // if let Some(remote_ip) = request.connection_info().remote() {
        
    // }

    let worker_info = {
        let state_read = state.read().unwrap();

        let action = match state_read.action {
            Action::Update => WorkerAction::Update,
            Action::Start(ref start_info) => {
                let start_info = state_read.start_infos
                    .get(start_info)
                    .expect("Start info must be present");

                WorkerAction::Start(start_info.clone())
            },
            Action::Stop => WorkerAction::Stop
        };

        WorkerInfo::new(state_read.version, state_read.update_version, action)
    };

    web::Json(worker_info)
}