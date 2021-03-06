use crate::_tests::_helpers::input_simulator::InputSimulator;
use crate::input::Input;
use crate::input::move_direction::MoveDirection::*;

#[test]
fn initially_it_says_no_movement() {
    let input = InputSimulator::new();
    assert!(!input.wants_to_move(Left));
    assert!(!input.wants_to_move(Right));
    assert!(!input.wants_to_move(Up));
    assert!(!input.wants_to_move(Down));
}

#[test]
fn simulate_move_x_causes_move_x_to_return_true() {
    let mut input = InputSimulator::new();
    input.simulate_move(Left);
    assert!(input.wants_to_move(Left));

    input.simulate_move(Right);
    assert!(input.wants_to_move(Right));

    input.simulate_move(Up);
    assert!(input.wants_to_move(Up));


    input.simulate_move(Down);
    assert!(input.wants_to_move(Down));

}

#[test]
fn simulate_move_x_causes_move_x_to_return_true_new_api() {
    let mut input = InputSimulator::new();
    input.simulate_move(Left);
    assert!(input.wants_to_move(Left));

    input.simulate_move(Right);
    assert!(input.wants_to_move(Right));

    input.simulate_move(Up);
    assert!(input.wants_to_move(Up));


    input.simulate_move(Down);
    assert!(input.wants_to_move(Down));

}


#[test]
fn simulate_move_x_resets_all_previous_simulate_calls() {
    let mut input = InputSimulator::new();

    input.simulate_move(Left);
    input.simulate_move(Right);

    assert!(!input.wants_to_move(Left));
    assert!(input.wants_to_move(Right));
}