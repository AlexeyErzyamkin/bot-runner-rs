use std::path::Path;
use std::io;
use std::fs;
use std::sync::RwLock;
use std::thread;
use std::collections::HashMap;

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
    WorkerAction,
    StartInfo
};
use shared::archiving;
use shared::{ URL_SCOPE, URL_STATE, URL_UPDATE };

mod state;

use state::{State, Action};

const PATH_UPDATES: &str = "data/updates";

#[derive(Deserialize)]
struct MasterConfig {
    addr: String,
    data_path: String,
    start_infos: HashMap<String, StartInfo>
}

fn get_state(state: web::Data<RwLock<State>>) -> impl Responder {
    let state_read = state.read().unwrap();

    let worker_info = WorkerInfo {
        version: state_read.version,
        update_version: state_read.update_version,
        action: match state_read.action {
            Action::Update => WorkerAction::Update,
            Action::Start(ref start_info) => {
                let start_info = state_read.start_infos
                    .get(start_info)
                    .expect("Start info must be present");

                WorkerAction::Start(start_info.clone())
            },
            Action::Stop => WorkerAction::Stop
        }
    };

    web::Json(worker_info)
}

fn get_update(state: web::Data<RwLock<State>>) -> impl Responder {
    let state_read = state.read().unwrap();

    if let Some(update_file) = &state_read.update_file {
        return Some(NamedFile::open(update_file));
    }

    None
}

fn main() -> io::Result<()> {
    prepare_environment()?;

    let config_path = Path::new("./data/master_config.json");
    let config: MasterConfig = shared::read_config(config_path)?;

    if config.start_infos.is_empty() {
        eprintln!("Start infos collection is empty");

        return Err(io::ErrorKind::InvalidData.into());
    }

    let data = web::Data::new(RwLock::new(State::new(config.start_infos)));

    handle_input(data.clone(), config.data_path);

    let handlers = move || {
        App::new()
            .register_data(data.clone())
            .service(
                web::scope(URL_SCOPE)
                    .service(
                        web::resource(URL_STATE).to(get_state)
                    )
                    .service(
                        web::resource(URL_UPDATE).to(get_update)
                    )
            )
    };

    println!("Master started");

    HttpServer::new(handlers)
        .bind(config.addr)?
        .run()

    // Ok(())
}

fn prepare_environment() -> io::Result<()> {
    fs::create_dir_all(PATH_UPDATES)?;
    
    Ok(())
}

fn handle_input(state: web::Data<RwLock<State>>, data_path: String) {
    thread::spawn(move || {
        loop {
            let mut buf = String::new();
            let len = io::stdin().read_line(&mut buf).expect("Read input failed");

            if len > 0 {
                let mut cmd = buf.split_whitespace();

                match cmd.next() {
                    Some("u") => {
                        let update_file = format!("{}/{}.zip", PATH_UPDATES, (state.read().unwrap()).update_version.0 + 1);

                        print!("Archiving to '{}'... ", update_file);
                        
                        archiving::archive_data(&data_path, &update_file).expect("Archiving failed");
                        
                        println!("Done");

                        state.write().unwrap().update(update_file);
                    },
                    Some("r") => {
                        match cmd.next() {
                            Some(si) => {
                                let key_exists = state.read().unwrap().start_infos.contains_key(si);
                                if key_exists {
                                    state.write().unwrap().start(si.to_string());
                                } else {
                                    eprintln!("Key {} not present in start infos", si);
                                }
                            },
                            None => {
                                let mut state_write = state.write().unwrap();
                                match state_write.last_start_info.take() {
                                    Some(lsi) => {
                                        state_write.start(lsi);
                                    },
                                    None => {
                                        let first_key = state_write.start_infos.keys().cloned().next().expect("Start infos collection is empty");

                                        state_write.start(first_key.to_owned());
                                    }
                                }
                            }
                        }
                    },
                    Some("s") => state.write().unwrap().stop(),
                    _ => {
                        eprintln!("Invalid command: {}", &buf);

                        continue;
                    }
                };
            }
        }
    });
}