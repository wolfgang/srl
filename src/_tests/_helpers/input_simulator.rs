use crate::input::Input;
use crate::input::move_direction::MoveDirection;

pub struct InputSimulator {
    simulating_move_left: bool,
    simulating_move_right: bool,
    simulating_move_up: bool,
    simulating_move_down: bool,
    simulated_direction: Option<MoveDirection>,
}


impl InputSimulator {
    pub fn new() -> Self {
        Self {
            simulated_direction: None,
            simulating_move_left: false,
            simulating_move_right: false,
            simulating_move_up: false,
            simulating_move_down: false
        }
    }

    pub fn simulate_move(&mut self, direction: MoveDirection) {
        self.simulated_direction = Some(direction)
    }
}

impl Input for InputSimulator {
    fn move_left(&self) -> bool {
        self.simulating_move_left
    }

    fn move_right(&self) -> bool {
        self.simulating_move_right
    }

    fn move_up(&self) -> bool {
        self.simulating_move_up
    }

    fn move_down(&self) -> bool {
        self.simulating_move_down
    }

    fn quit_game(&self) -> bool {
        false
    }

    fn wants_to_move(&self, direction: MoveDirection) -> bool {
        match self.simulated_direction {
            Some(value) => {direction == value}
            None => false
        }
    }
}