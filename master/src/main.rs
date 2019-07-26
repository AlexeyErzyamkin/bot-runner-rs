use std::path::Path;
use std::io;
use std::fs;
use std::sync::{
    RwLock,
    Arc
};
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

use zip::{
    ZipWriter,
    CompressionMethod,
    write::FileOptions
};

use shared;
use shared::models::{
    WorkerInfo,
    WorkerAction,
    StartInfo
};

mod state;

use state::{State, Action};

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

fn main() -> std::io::Result<()> {
    let config_path = Path::new("./data/master_config.json");
    let config: MasterConfig = shared::read_config(config_path)
        .map_err(|e| {
            eprintln!("Error reading config");
            e
        })?;

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

    // Ok(())
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
                        let update_file = format!("./data/updates/update{}.zip", (state.read().unwrap()).update_version + 1);

                        print!("Archiving to '{}'... ", update_file);
                        
                        archive_data(&data_path, &update_file).expect("Archiving failed");
                        
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

        unreachable!()
    });
}

fn archive_data(path: &str, out_path: &str) -> io::Result<()> {
    let arc_file = fs::File::create(out_path)?;
    let mut zip = ZipWriter::new(arc_file);

    let options = FileOptions::default()
        .compression_method(CompressionMethod::Stored);

    arc_dir(Path::new(path), path, options, &mut zip);

    zip.finish()?;

    Ok(())
}

fn arc_dir<P: AsRef<Path>>(path: P, prefix: &str, options: FileOptions, mut zip: &mut ZipWriter<fs::File>) {
    for each_file in fs::read_dir(path).unwrap() {
        let each_file = each_file.unwrap();
        let each_file_path = each_file.path();
        let file_type = each_file.file_type().unwrap();

        let path = each_file_path.strip_prefix(Path::new(prefix)).unwrap();

        if file_type.is_dir() {
            // println!("Dir: {:?}", path);

            zip.add_directory_from_path(path, options).unwrap();

            arc_dir(each_file_path, prefix, options, &mut zip)
        } else if file_type.is_file() {
            // println!("File: {:?}", path);

            zip.start_file_from_path(path, options).unwrap();
            let mut from_file = fs::File::open(each_file_path).unwrap();
            io::copy(&mut from_file, &mut zip).unwrap();
        }
    }
}