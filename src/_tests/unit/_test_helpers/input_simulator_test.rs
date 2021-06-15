use crate::_tests::_helpers::input_simulator::InputSimulator;
use crate::input::Input;

#[test]
fn initially_it_says_no_movement() {
    let input = InputSimulator::new();

    assert!(!input.move_left());
    assert!(!input.move_right());
    assert!(!input.move_up());
    assert!(!input.move_down());
}

#[test]
fn simulate_move_x_causes_move_x_to_return_true() {
    let mut input = InputSimulator::new();
    input.simulate_move_left();
    assert!(input.move_left());

    input.simulate_move_right();
    assert!(input.move_right());

    input.simulate_move_up();
    assert!(input.move_up());

    input.simulate_move_down();
    assert!(input.move_down());
}

#[test]
fn simulate_move_x_resets_all_previous_simulate_calls() {
    let mut input = InputSimulator::new();

    input.simulate_move_left();
    input.simulate_move_right();

    assert!(!input.move_left());
    assert!(input.move_right());
}