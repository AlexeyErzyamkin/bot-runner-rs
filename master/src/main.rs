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

use actix_files::NamedFile;

use serde::Deserialize;

use shared;
use shared::models::{
    WorkerInfo,
    WorkerAction
};

mod state;

use state::{State, Action};

#[derive(Deserialize)]
struct MasterConfig {
    addr: String,
    data_path: String
}

fn get_state(state: web::Data<RwLock<State>>) -> impl Responder {
    let state_read = state.read().unwrap();

    let worker_info = WorkerInfo {
        version: state_read.version,
        action: match state_read.action {
            Action::Update => WorkerAction::Update,
            Action::Start => WorkerAction::Start,
            Action::Stop => WorkerAction::Stop
        },
        update_url: "".to_string(),
        start_command_line: "".to_string()
    };

    web::Json(worker_info)
}

fn get_update() -> impl Responder {
    NamedFile::open("tasks.txt")
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
                    .service(
                        web::resource("/update").to(get_update)
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