use std::collections::HashMap;

use shared::models::StartInfo;

#[derive(PartialEq)]
pub enum Action {
    Stop,
    Start(String),
    Update
}

pub struct State {
    pub version: u32,
    pub update_version: u32,
    pub action: Action,
    pub update_file: Option<String>,
    pub last_start_info: Option<String>,
    pub start_infos: HashMap<String, StartInfo>
}

impl State {
    pub fn new(start_infos: HashMap<String, StartInfo>) -> Self {
        Self {
            version: 0,
            update_version: 0,
            action: Action::Stop,
            update_file: None,
            last_start_info: None,
            start_infos
        }
    }

    pub fn start(&mut self, start_info: String) {
        self.set_action(Action::Start(start_info));
    }

    pub fn stop(&mut self) {
        self.set_action(Action::Stop);
    }

    pub fn update(&mut self, update_file: String) {
        self.version += 1;
        self.update_version += 1;
        self.update_file = Some(update_file);

        self.action = Action::Update;
    }

    fn set_action(&mut self, action: Action ) { //, start_info: Option<String>) {
        if self.action != action {
            self.version += 1;
            self.action = action;
            // self.start_info = start_info;
        }
    }
}

// impl Default for State {
//     fn default() -> Self {
//         Self {
//             version: 0,
//             update_version: 0,
//             action: Action::Stop,
//             update_file: None
//         }
//     }
// }