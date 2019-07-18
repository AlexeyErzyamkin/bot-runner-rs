use std::path::Path;
use std::io;
use std::sync::{
    mpsc,
    mpsc::{Sender, Receiver}
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

#[derive(Deserialize)]
struct MasterConfig {
    addr: String,
    data_path: String
}

enum UserCommand {
    Update,
    Start,
    Stop
}

fn index() -> impl Responder {
    format!("Hello")
}

fn index2(arg: web::Path<u32>) -> impl Responder {
    format!("Hello {}", arg)
}

// fn download() -> impl Responder {
    
// }

fn main() -> std::io::Result<()> {
    let config_path = Path::new("./data/master_config.json");
    let config: MasterConfig = shared::read_config(config_path)?;

    let (itx, irx) = mpsc::channel();

    input(itx);

    let handlers = || {
        App::new()
            .service(
                web::resource("/check_update").to(index)
            )
            .service(
                web::resource("/{id}").route(
                    web::get().to(index2)
                )
            )
    };

    HttpServer::new(handlers)
        .bind(config.addr)?
        .run()
}

fn q(rx: mpsc::Receiver<UserCommand>) {

}

fn input(tx: mpsc::Sender<UserCommand>) /*-> io::Result<()>*/ {
    thread::spawn(move || {
        let mut buf = String::new();

        loop {
            let len = io::stdin().read_line(&mut buf).expect("Read input failed");

            if len > 0 {
                let command = match &buf[..0] {
                    "u" => UserCommand::Update,
                    "r" => UserCommand::Start,
                    "s" => UserCommand::Stop,
                    _ => {
                        eprintln!("Invalid command");

                        continue;
                    }
                };

                tx.send(command).expect("Use command sending failed");
            }
        }

        unreachable!()
    });
}