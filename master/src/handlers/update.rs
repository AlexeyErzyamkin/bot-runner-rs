use std::sync::RwLock;

use actix_web::{
    Responder,
    web
};

use actix_files::NamedFile;

use crate::state::State;

pub async fn handle_update(state: web::Data<RwLock<State>>) -> impl Responder {
    let state_read = state.read().unwrap();

    if let Some(update_file) = &state_read.update_file {
        return Some(NamedFile::open(update_file));
    }

    None
}