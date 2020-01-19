pub mod error;

use shared::models::{ StartInfo, UpdateVersion };

pub use error::Error;

pub enum WorkerCommand {
    Stop,
    Start(StartInfo),
    Update(UpdateVersion),
    Quit
}

pub type Result<T> = std::result::Result<T, Error>;