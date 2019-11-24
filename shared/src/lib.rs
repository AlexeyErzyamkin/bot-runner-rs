mod config_reader;
mod constants;

pub mod archiving;
pub mod models;

pub use config_reader::read_config;
pub use constants::{ URL_SCOPE, URL_STATE, URL_UPDATE, URL_REGISTER };