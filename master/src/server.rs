use std::{
    io,
    sync::RwLock
};

use {
    actix,
    actix::Actor,
    actix_web::{
        HttpServer,
        App,
        web,
    }
};

use {
    shared::{URL_SCOPE, URL_STATE, URL_UPDATE, URL_REGISTER},
    crate::{
        actors::{
            master::MasterActor
        },
        state::State,
        handlers
    }
};

pub struct ServerState {
    pub master_addr: actix::Addr<MasterActor>
}

pub fn run(data: web::Data<RwLock<State>>, addr: String) -> io::Result<()> {
    let sys = actix::System::new("bot-runner");

    let master_addr = MasterActor::new().start();

    let handlers = move || {
        let state = ServerState {
            master_addr: master_addr.clone()
        };

        App::new()
            .register_data(data.clone())
            .service(
                web::scope("/v2")
                    .data(state)
                    .route(URL_REGISTER, web::post().to_async(handlers::handle_register))
                    .route(URL_STATE, web::post().to_async(handlers::handle_state_v2))
            )
            .service(
                web::scope(URL_SCOPE)
                    .route(URL_STATE, web::get().to(handlers::handle_state))
                    .route(URL_UPDATE, web::get().to(handlers::handle_update))
            )
    };

    let _server = HttpServer::new(handlers)
        .bind(addr)?
        .start();

    sys.run()
}