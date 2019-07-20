use actix_rt::System;
use actix_web::client::Client;

use futures::{
    Future,
    lazy
};

fn main() {
    System::new("worker").block_on(lazy(|| {
        let client = Client::default();

        client
            .get("http://127.0.0.1:8081/bot-runner/state")
            .send()
            .map_err(|_| ())
            .and_then(|response| {
                println!("R: {:?}", response);

                Ok(())
            })
    })).unwrap();
}
