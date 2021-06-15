use crate::_tests::_helpers::input_simulator::InputSimulator;
use crate::input::Input;
use crate::input::move_direction::MoveDirection::*;

#[test]
fn initially_it_says_no_movement() {
    let input = InputSimulator::new();

    assert!(!input.move_left());
    assert!(!input.move_right());
    assert!(!input.move_up());
    assert!(!input.move_down());
    assert!(!input.wants_to_move(Left));
    assert!(!input.wants_to_move(Right));
    assert!(!input.wants_to_move(Up));
    assert!(!input.wants_to_move(Down));
}

#[test]
fn simulate_move_x_causes_move_x_to_return_true() {
    let mut input = InputSimulator::new();
    input.simulate_move_left();
    assert!(input.wants_to_move(Left));

    input.simulate_move_right();
    assert!(input.wants_to_move(Right));

    input.simulate_move_up();
    assert!(input.wants_to_move(Up));


    input.simulate_move_down();
    assert!(input.wants_to_move(Down));

}

#[test]
fn simulate_move_x_resets_all_previous_simulate_calls() {
    let mut input = InputSimulator::new();

    input.simulate_move_left();
    input.simulate_move_right();

    assert!(!input.wants_to_move(Left));
    assert!(input.wants_to_move(Right));
}