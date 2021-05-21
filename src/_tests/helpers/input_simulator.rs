use crate::input::Input;

pub struct InputSimulator {}

impl InputSimulator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for InputSimulator {
    fn move_left(&self) -> bool {
        false
    }

    fn move_right(&self) -> bool {
        false
    }

    fn move_up(&self) -> bool {
        false
    }

    fn move_down(&self) -> bool {
        false
    }
}