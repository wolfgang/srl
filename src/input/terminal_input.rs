use console::Key;

use crate::input::Input;

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
    fn move_left(&self) -> bool {
        self.pressed_key == Key::ArrowLeft
    }

    fn move_right(&self) -> bool {
        self.pressed_key == Key::ArrowRight
    }

    fn move_up(&self) -> bool {
        self.pressed_key == Key::ArrowUp
    }

    fn move_down(&self) -> bool {
        self.pressed_key == Key::ArrowDown
    }
}