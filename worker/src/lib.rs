use shared::models::{ StartInfo, UpdateVersion };

pub enum WorkerCommand {
    Stop,
    Start(StartInfo),
    Update(UpdateVersion),
    Quit
}