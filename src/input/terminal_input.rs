use console::Key;

use crate::input::Input;
use crate::input::move_direction::MoveDirection;
use crate::input::move_direction::MoveDirection::*;

pub struct TerminalInput {
    pressed_key: Key
}

impl TerminalInput {
    pub fn new() -> Self {
        Self {pressed_key: Key::Unknown}
    }

    pub fn on_key(&mut self, key: Key) {
        self.pressed_key = key;
    }
}

impl Input for TerminalInput {
    fn quit_game(&self) -> bool {
        self.pressed_key == Key::Escape
    }

    fn wants_to_move(&self, direction: MoveDirection) -> bool {
        match direction {
            Left => {self.pressed_key == Key::ArrowLeft}
            Right => {self.pressed_key == Key::ArrowRight}
            Up => {self.pressed_key == Key::ArrowUp}
            Down => {self.pressed_key == Key::ArrowDown}
        }
    }
}