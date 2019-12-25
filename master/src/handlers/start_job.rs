// use {
//     ::actix::prelude::*,
//     ::actix_web::{
//         web,
//         HttpResponse,
//         ResponseError
//     },
//     shared::models::{
//         StartJobRequest,
//         EmptyResponse
//     },
//     crate::{
//         actors::{
//             Error,
//             master::messages::StartJob
//         },
//         server::ServerState
//     }
// };

// pub fn handle_start_job((request, state): (web::Json<StartJobRequest>, web::Data<ServerState>))
//     -> impl Future<Item = HttpResponse, Error = Error>
// {
//     state.master_addr
//         .send(StartJob {})
//         .from_err()
//         .and_then(|res| match res {
//             Ok(_res) => {
//                 Ok(HttpResponse::Ok().json(EmptyResponse))
//             },
//             Err(e) => Ok(e.error_response())
//         })
// }