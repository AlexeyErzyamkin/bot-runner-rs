use std::path::Path;
use std::io;
use std::sync::{
    RwLock,
    Arc
};
use std::thread;
use std::default::Default;

use actix_web::{
    HttpServer,
    Responder,
    App,
    web
};

use serde::Deserialize;

use shared;

#[derive(Deserialize)]
struct MasterConfig {
    addr: String,
    data_path: String
}

// enum UserCommand {
//     Update,
//     Start,
//     Stop
// }

#[derive(PartialEq)]
enum Action {
    Stop,
    Start,
    Update
}

struct State {
    version: u32,
    action: Action
}

impl State {
    pub fn update(&mut self, action: Action) {
        if self.action != action {
            self.version += 1;
            self.action = action;
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            version: 0,
            action: Action::Stop
        }
    }
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

    HttpServer::new(handlers)
        .bind(config.addr)?
        .run()
}

fn handle_input(state: web::Data<RwLock<State>>) {
    thread::spawn(move || {
        let mut buf = String::new();

        loop {
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