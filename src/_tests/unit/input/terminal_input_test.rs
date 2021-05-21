use console::Key;

use crate::input::{Input, TerminalInput};

#[test]
fn on_key_translates_cursor_keys_to_movement() {
    let mut input = TerminalInput::new();

    input.on_key(Key::ArrowLeft);
    assert!(input.move_left());

    assert!(!input.move_right());
    assert!(!input.move_up());
    assert!(!input.move_down());

    input.on_key(Key::ArrowRight);
    assert!(input.move_right());

    assert!(!input.move_left());
    assert!(!input.move_up());
    assert!(!input.move_down());

    input.on_key(Key::ArrowUp);
    assert!(input.move_up());

    assert!(!input.move_left());
    assert!(!input.move_right());
    assert!(!input.move_down());

    input.on_key(Key::ArrowDown);
    assert!(input.move_down());

    assert!(!input.move_left());
    assert!(!input.move_right());
    assert!(!input.move_up());
}

#[test]
fn on_key_translates_esc_to_quit_game() {
    let mut input = TerminalInput::new();

    assert!(!input.quit_game());
    input.on_key(Key::Escape);
    assert!(input.quit_game());

}