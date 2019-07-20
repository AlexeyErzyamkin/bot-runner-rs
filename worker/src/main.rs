// use actix_rt::System;
// use actix_web::client::Client;

use reqwest;

use futures::{
    Future,
    lazy
};

fn main() -> std::result::Result<(), Box<::std::error::Error>> {
    // System::new("worker").block_on(lazy(|| {
    //     let client = Client::default();

    //     client
    //         .get("http://127.0.0.1:8081/bot-runner/state")
    //         .send()
    //         .map_err(|_| ())
    //         .and_then(|response| {
    //             println!("R: {:?}", response);

    //             Ok(())
    //         })
    // })).unwrap();

    let response_text = reqwest::get("http://127.0.0.1:8081/bot-runner/state")?.text()?;

    println!("R: {}", response_text);

    Ok(())
}

fn update_status() {
    match reqwest::get("http://127.0.0.1:8081/bot-runner/state") {
        
    }
}