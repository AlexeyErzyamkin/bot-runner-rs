use std::io;
use std::io::Write;
use std::sync::RwLock;
use std::thread;

use shared;
use shared::archiving;

use actix_web::web;

use tokio::fs;

use master::config;
use master::server;
use master::state::State;
use master::Result;

const PATH_UPDATES: &str = "data/updates";
const PATH_CONFIG: &str = "./data/master_config.json";

fn print_help() {
    println!("BotRunner-RS by Alexey V. Erzyamkin");
    println!("   Commands:");
    println!("      h - this help");
    println!("      u - update files");
    println!("      c - read configurations");
    println!("      r <name> - run configuration with <name>");
    println!("      s - stop");
}

#[tokio::main]
async fn main() -> Result<()> {
    prepare_environment().await?;
    print_help();

    let config = config::read(PATH_CONFIG)?;

    let data = web::Data::new(RwLock::new(State::new(config.start_infos)));

    handle_input(data.clone(), config.data_path);
    server::run(data, config.addr).await
}

async fn prepare_environment() -> Result<()> {
    fs::create_dir_all(PATH_UPDATES).await?;

    Ok(())
}

fn handle_input(state: web::Data<RwLock<State>>, data_path: String) {
    thread::spawn(move || loop {
        let mut buf = String::new();

        print!("# ");
        io::stdout().flush().unwrap();

        let len = io::stdin().read_line(&mut buf).expect("Read input failed");

        if len > 0 {
            let mut cmd = buf.split_whitespace();

            match cmd.next() {
                Some("h") => {
                    print_help();
                }
                Some("c") => {
                    if let Ok(new_config) = config::read(PATH_CONFIG) {
                        state.write().unwrap().start_infos = new_config.start_infos;

                        println!("Successfully read config file");
                    } else {
                        eprintln!("Error reading config file");
                    }
                }
                Some("u") => {
                    let update_file = format!(
                        "{}/{}.zip",
                        PATH_UPDATES,
                        state.read().unwrap().update_version.next()
                    );

                    println!("Archiving to '{}'... ", update_file);

                    archiving::archive_data(&data_path, &update_file).expect("Archiving failed");

                    println!("Done");

                    state.write().unwrap().update(update_file);
                }
                Some("r") => match cmd.next() {
                    Some(si) => {
                        let key_exists = state.read().unwrap().start_infos.contains_key(si);
                        if key_exists {
                            state.write().unwrap().start(si.to_string());

                            println!("Start scheduled with key {}", si);
                        } else {
                            eprintln!("Key {} not present in start infos", si);
                        }
                    }
                    None => {
                        let mut state_write = state.write().unwrap();
                        match state_write.last_start_info.take() {
                            Some(lsi) => {
                                state_write.start(lsi.to_owned());

                                println!("Start scheduled with last used key {}", lsi);
                            }
                            None => {
                                let first_key = state_write
                                    .start_infos
                                    .keys()
                                    .cloned()
                                    .next()
                                    .expect("Start infos collection is empty");

                                state_write.start(first_key.to_owned());

                                println!("Start scheduled with first key {}", first_key);
                            }
                        }
                    }
                }
                Some("s") => {
                    state.write().unwrap().stop();

                    println!("Stop scheduled");
                }
                _ => {
                    eprintln!("Invalid command: {}", &buf);

                    continue;
                }
            };
        }
    });
}
