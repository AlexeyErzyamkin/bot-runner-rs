use {
    ::actix::prelude::*,
    ::actix_web::{
        web,
        HttpResponse,
        ResponseError
    },
    ::tracing::{
        span, Level, info
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

pub async fn handle_register(state: web::Data<ServerState>) -> Result<HttpResponse, Error>
    // -> impl Future<Item = HttpResponse, Error = Error>
{
    let t_span = span!(Level::TRACE, "/register");
    let _t_enter = t_span.enter();

    let res = state.master_addr.send(RegisterWorker {}).await?;

    match res {
        Ok(res) => Ok(HttpResponse::Ok().json(RegisterResponse {
            key: res.key
        })),
        Err(e) => Ok(e.error_response())
    }

        // .from_err()
        // .and_then(|res| {
        //     info!("Registered worker");

        //     match res {
        //         Ok(res) => Ok(HttpResponse::Ok().json(RegisterResponse {
        //             key: res.key
        //         })),
        //         Err(e) => Ok(e.error_response())
        //     }
        // })
}