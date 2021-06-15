use console::Key;

use crate::input::{Input, TerminalInput};
use crate::input::move_direction::MoveDirection::*;

#[test]
fn on_key_translates_cursor_keys_to_movement() {
    let mut input = TerminalInput::new();

    input.on_key(Key::ArrowLeft);
    assert!(input.wants_to_move(Left));

    assert!(!input.wants_to_move(Right));
    assert!(!input.wants_to_move(Up));
    assert!(!input.wants_to_move(Down));

    input.on_key(Key::ArrowRight);
    assert!(input.wants_to_move(Right));

    assert!(!input.wants_to_move(Left));
    assert!(!input.wants_to_move(Up));
    assert!(!input.wants_to_move(Down));

    input.on_key(Key::ArrowUp);
    assert!(input.wants_to_move(Up));

    assert!(!input.wants_to_move(Left));
    assert!(!input.wants_to_move(Right));
    assert!(!input.wants_to_move(Down));

    input.on_key(Key::ArrowDown);
    assert!(input.wants_to_move(Down));

    assert!(!input.wants_to_move(Left));
    assert!(!input.wants_to_move(Right));
    assert!(!input.wants_to_move(Up));
}

#[test]
fn on_key_translates_esc_to_quit_game() {
    let mut input = TerminalInput::new();

    assert!(!input.quit_game());
    input.on_key(Key::Escape);
    assert!(input.quit_game());

}