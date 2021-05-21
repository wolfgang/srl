use crate::_tests::helpers::input_simulator::InputSimulator;
use crate::input::Input;

#[test]
fn initially_it_says_no_movement() {
    let input = InputSimulator::new();

    assert!(!input.move_left());
    assert!(!input.move_right());
    assert!(!input.move_up());
    assert!(!input.move_down());
}