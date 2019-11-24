use std::collections::{HashMap};

use shared::models::{ StartInfo, UpdateVersion, StateVersion };
use crate::{ WorkerInfo, WorkerAddr };

#[derive(PartialEq)]
pub enum Action {
    Stop,
    Start(String),
    Update
}

/// Holds current state of master
pub struct State {
    pub version: StateVersion,
    pub update_version: UpdateVersion,
    pub action: Action,
    pub update_file: Option<String>,
    pub last_start_info: Option<String>,
    pub start_infos: HashMap<String, StartInfo>,
    pub worker_infos: HashMap<WorkerAddr, WorkerInfo>
}

impl State {
    pub fn new(start_infos: HashMap<String, StartInfo>) -> Self {
        Self {
            version: StateVersion::default(),
            update_version: UpdateVersion::default(),
            action: Action::Stop,
            update_file: None,
            last_start_info: None,
            start_infos,
            worker_infos: HashMap::new()
        }
    }

    pub fn start(&mut self, start_info: String) {
        self.set_action(Action::Start(start_info));
    }

    pub fn stop(&mut self) {
        self.set_action(Action::Stop);
    }

    pub fn update(&mut self, update_file: String) {
        self.version.0 += 1;
        self.update_version.0 += 1;
        self.update_file = Some(update_file);

        self.action = Action::Update;
    }

    fn set_action(&mut self, action: Action ) {
        if self.action != action {
            self.version.0 += 1;
            self.action = action;
        }
    }
}