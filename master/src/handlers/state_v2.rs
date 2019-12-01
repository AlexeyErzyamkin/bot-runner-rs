use {
    ::actix::prelude::*,
    ::actix_web::{
        web,
        HttpResponse,
        ResponseError
    },
    shared,
    shared::models::{
        StateRequest,
        StateResponse
    },
    crate::{
        actors::{
            Error,
            master::messages::GetWorkerState
        },
        server::ServerState
    }
};

pub fn handle_state_v2((request, state): (web::Json<StateRequest>, web::Data<ServerState>))
    -> impl Future<Item = HttpResponse, Error = Error>
{
    state.master_addr
        .send(GetWorkerState { worker_uid: request.id.clone() })
        .from_err()
        .and_then(|res| match res {
            Ok(_res) => {
                Ok(HttpResponse::Ok().json(StateResponse {}))
            },
            Err(e) => Ok(e.error_response())
        })
}