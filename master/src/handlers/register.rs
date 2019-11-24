use actix::{
    prelude::*
};

use actix_web::{
    web,
    HttpResponse
};

use shared;
use shared::models::{
    RegisterRequest,
    RegisterResponse
};

use crate::{
    actors::{
        Error,
        master::RegisterWorker
    },
    server::ServerState
};

pub fn handle_register((_request, state): (web::Json<RegisterRequest>, web::Data<ServerState>))
    -> impl Future<Item = HttpResponse, Error = Error> { //Responder {
    state.master_addr
        .send(RegisterWorker {})
        .from_err()
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(RegisterResponse {
                id: res.id
            })),
            Err(e) => Ok(e.error_response())
        })
}