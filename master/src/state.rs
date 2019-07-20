use std::default::Default;

#[derive(PartialEq)]
pub enum Action {
    Stop,
    Start,
    Update
}

pub struct State {
    pub version: u32,
    pub action: Action
}

impl State {
    pub fn update(&mut self, action: Action) {
        if self.action != action {
            self.version += 1;
            self.action = action;
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            version: 0,
            action: Action::Stop
        }
    }
}