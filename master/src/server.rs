use std::io;
use std::sync::RwLock;

use actix_web::{
    HttpServer,
    App,
    web
};

use shared::{ URL_SCOPE, URL_STATE, URL_UPDATE };

use crate::state::State;
use crate::handlers;

pub fn run(data: web::Data<RwLock<State>>, addr: String) -> io::Result<()> {
    let handlers = move || {
        App::new()
            .register_data(data.clone())
            .service(
                web::scope(URL_SCOPE)
                    .service(
                        web::resource(URL_STATE).to(handlers::handle_state)
                    )
                    .service(
                        web::resource(URL_UPDATE).to(handlers::handle_update)
                    )
            )
    };

    HttpServer::new(handlers)
        .bind(addr)?
        .run()
}