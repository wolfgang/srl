use crate::input::Input;
use crate::input::move_direction::MoveDirection;

pub struct InputSimulator {
    simulated_direction: Option<MoveDirection>,
}


impl InputSimulator {
    pub fn new() -> Self {
        Self { simulated_direction: None }
    }

    pub fn simulate_move(&mut self, direction: MoveDirection) {
        self.simulated_direction = Some(direction)
    }
}

impl Input for InputSimulator {
    fn quit_game(&self) -> bool {
        false
    }

    fn wants_to_move(&self, direction: MoveDirection) -> bool {
        match self.simulated_direction {
            Some(value) => { direction == value }
            None => false
        }
    }
}