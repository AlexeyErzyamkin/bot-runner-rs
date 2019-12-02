use {
    ::actix::prelude::*,
    ::actix_web::{
        web,
        HttpResponse,
        ResponseError
    },
    shared,
    shared::models::{
        RegisterResponse
    },
    crate::{
        actors::{
            Error,
            master::messages::RegisterWorker
        },
        server::ServerState
    }
};

pub fn handle_register(state: web::Data<ServerState>)
    -> impl Future<Item = HttpResponse, Error = Error>
{
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