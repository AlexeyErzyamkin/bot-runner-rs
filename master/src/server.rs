use std::sync::RwLock;

use actix_web::{
    HttpServer,
    App,
    web
};

use shared::{ URL_SCOPE, URL_STATE, URL_UPDATE };

use crate::state::State;
use crate::handlers;
use crate::Result;

pub async fn run(data: web::Data<RwLock<State>>, addr: String) -> Result<()> {
    let handlers = move || {
        App::new()
            .data(data.clone())
            .service(
                web::scope(URL_SCOPE)
                    .service(web::resource(URL_STATE).to(handlers::handle_state))
                    .service(web::resource(URL_UPDATE).to(handlers::handle_update))
            )
    };

    HttpServer::new(handlers)
        .bind(addr)?
        .run()
        .await
        .or_else(|e| Result::Err(e.into()))
}