use {
    ::actix::prelude::*,
    ::actix_web::{
        web,
        HttpResponse,
        ResponseError
    },
    shared::{
        self,
        models::{
            StateRequest,
            StateResponse
        }
    },
    crate::{
        actors::{
            Error,
            master::messages::GetWorkerState
        },
        server::ServerState
    }
};

pub async fn handle_state_v2((request, state): (web::Json<StateRequest>, web::Data<ServerState>))
    -> Result<HttpResponse, Error>
{
    let res = state.master_addr.send(GetWorkerState { worker_key: request.key.clone() }).await?;
    
    match res {
        Ok(_res) => {
            Ok(HttpResponse::Ok().json(StateResponse {}))
        },
        Err(e) => Ok(e.error_response())
    }
}