use crate::input::Input;

#[derive(Default)]
pub struct InputSimulator {
    simulating_move_left: bool,
    simulating_move_right: bool,
    simulating_move_up: bool,
    simulating_move_down: bool,
}


impl InputSimulator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn simulate_move_left(&mut self) {
        self.reset();
        self.simulating_move_left = true
    }

    pub fn simulate_move_right(&mut self) {
        self.reset();
        self.simulating_move_right = true
    }

    pub fn simulate_move_up(&mut self) {
        self.reset();
        self.simulating_move_up = true
    }

    pub fn simulate_move_down(&mut self) {
        self.reset();
        self.simulating_move_down = true
    }

    fn reset(&mut self) {
        self.simulating_move_left = false;
        self.simulating_move_right = false;
        self.simulating_move_up = false;
        self.simulating_move_down = false;
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
}