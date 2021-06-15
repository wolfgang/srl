use crate::_tests::_helpers::testable_game::TestableGame;
use crate::input::move_direction::MoveDirection::*;

#[test]
fn player_moves_according_to_input() {
    let mut game = TestableGame::from_strings(vec![
        ". . . . .",
        ". . @ . .",
        ". . . . .",
    ]);

    game.render();
    game.renderer.assert_tiles(vec![
        ". . . . .",
        ". . @ . .",
        ". . . . .",
    ]);

    game.input.simulate_move(Right);
    game.verify_next_tiles(vec![
        ". . . . .",
        ". . . @ .",
        ". . . . .",
    ]);

    game.input.simulate_move(Down);
    game.verify_next_tiles(vec![
        ". . . . .",
        ". . . . .",
        ". . . @ .",
    ]);

    game.input.simulate_move(Left);
    game.verify_next_tiles(vec![
        ". . . . .",
        ". . . . .",
        ". . @ . .",
    ]);

    game.input.simulate_move(Up);
    game.verify_next_tiles(vec![
        ". . . . .",
        ". . @ . .",
        ". . . . .",
    ]);
}