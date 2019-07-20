use std::path::Path;
use std::io;
use std::sync::{
    RwLock,
    Arc
};
use std::thread;

use actix_web::{
    HttpServer,
    Responder,
    App,
    web
};

use serde::Deserialize;

use shared;

mod state;

use state::{State, Action};

#[derive(Deserialize)]
struct MasterConfig {
    addr: String,
    data_path: String
}

fn get_state(state: web::Data<RwLock<State>>) -> impl Responder {
    let state_read = state.read().unwrap();

    format!("state: v={}, a={}", state_read.version, match state_read.action {
        Action::Update => "update",
        Action::Start => "start",
        Action::Stop => "stop"
    })
}

fn main() -> std::io::Result<()> {
    let config_path = Path::new("./data/master_config.json");
    let config: MasterConfig = shared::read_config(config_path)?;

    let data = web::Data::new(RwLock::new(State::default()));

    handle_input(data.clone());

    let handlers = move || {
        App::new()
            .register_data(data.clone())
            .service(
                web::scope("/bot-runner")
                    .service(
                        web::resource("/state").to(get_state)
                    )
            )
    };

    println!("Master started");

    HttpServer::new(handlers)
        .bind(config.addr)?
        .run()
}

fn handle_input(state: web::Data<RwLock<State>>) {
    thread::spawn(move || {
        loop {
            let mut buf = String::new();
            let len = io::stdin().read_line(&mut buf).expect("Read input failed");

            if len > 0 {
                let cmd = &buf[..1];
                let action = match cmd {
                    "u" => Action::Update,
                    "r" => Action::Start,
                    "s" => Action::Stop,
                    _ => {
                        eprintln!("Invalid command: {}", cmd);

                        continue;
                    }
                };

                (*state.write().unwrap()).update(action);
            }
        }

        unreachable!()
    });
}