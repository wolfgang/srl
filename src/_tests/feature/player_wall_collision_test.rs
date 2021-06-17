use crate::_tests::_helpers::testable_game::TestableGame;
use crate::input::move_direction::MoveDirection::*;

#[test]
fn player_collides_with_north_wall() {
    let mut game = TestableGame::from_strings(vec![
        "# # #",
        ". @ .",
    ]);

    game.input.simulate_move(Up);
    game.verify_next_tiles(vec![
        "# # #",
        ". @ .",
    ]);
}

#[test]
fn player_collides_with_west_wall() {
    let mut game = TestableGame::from_strings(vec![
        "# .",
        "# @",
        "# .",
    ]);

    game.input.simulate_move(Left);
    game.verify_next_tiles(vec![
        "# .",
        "# @",
        "# .",
    ]);
}

#[test]
fn player_collides_with_east_wall() {
    let mut game = TestableGame::from_strings(vec![
        ". . #",
        ". @ #",
        ". . #",
    ]);

    game.input.simulate_move(Right);
    game.verify_next_tiles(vec![
        ". . #",
        ". @ #",
        ". . #",
    ]);
}