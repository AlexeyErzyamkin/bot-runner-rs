// use std::default::Default;

use shared::models::StartInfo;

#[derive(PartialEq)]
pub enum Action {
    Stop,
    Start,
    Update
}

pub struct State {
    pub version: u32,
    pub update_version: u32,
    pub action: Action,
    pub update_file: Option<String>,
    pub start_info: StartInfo
}

impl State {
    pub fn new(start_info: StartInfo) -> Self {
        Self {
            version: 0,
            update_version: 0,
            action: Action::Stop,
            update_file: None,
            start_info
        }
    }

    pub fn start(&mut self) {
        self.set_action(Action::Start);
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

    fn set_action(&mut self, action: Action) {
        if self.action != action {
            self.version += 1;
            self.action = action;
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