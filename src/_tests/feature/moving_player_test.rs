use crate::_tests::helpers::testable_game::TestableGame;

#[test]
fn player_moves_according_to_input() {
    let mut game = TestableGame::from_strings(vec![
        ". . . . .",
        ". . @ . .",
        ". . . . .",
    ]);

    game.render();
    game.renderer.assert_frame(vec![
        ". . . . .",
        ". . @ . .",
        ". . . . .",
    ]);

    game.input.simulate_move_right();
    game.verify_next_frame(vec![
        ". . . . .",
        ". . . @ .",
        ". . . . .",
    ]);

    game.input.simulate_move_down();
    game.verify_next_frame(vec![
        ". . . . .",
        ". . . . .",
        ". . . @ .",
    ]);

    game.input.simulate_move_left();
    game.verify_next_frame(vec![
        ". . . . .",
        ". . . . .",
        ". . @ . .",
    ]);

    game.input.simulate_move_up();
    game.verify_next_frame(vec![
        ". . . . .",
        ". . @ . .",
        ". . . . .",
    ]);
}