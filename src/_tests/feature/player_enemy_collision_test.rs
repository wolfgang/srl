use crate::_tests::_helpers::testable_game::TestableGame;
use crate::input::move_direction::MoveDirection::*;

#[test]
fn player_collides_with_enemy_to_the_right() {
    let mut game = TestableGame::from_strings(vec![
        "....",
        ".@E.",
        "...."
    ]);

    game.input.simulate_move(Right);
    game.verify_next_tiles(vec![
        "....",
        ".@E.",
        "...."
    ]);
}

#[test]
fn player_collides_with_enemy_to_the_left() {
    let mut game = TestableGame::from_strings(vec![
        "....",
        "E@..",
        "...."
    ]);

    game.input.simulate_move(Left);
    game.verify_next_tiles(vec![
        "....",
        "E@..",
        "...."
    ]);
}

#[test]
fn player_collides_with_enemy_above() {
    let mut game = TestableGame::from_strings(vec![
        ".E..",
        ".@..",
        "...."
    ]);

    game.input.simulate_move(Up);
    game.verify_next_tiles(vec![
        ".E..",
        ".@..",
        "...."
    ]);
}

#[test]
fn player_collides_with_enemy_below() {
    let mut game = TestableGame::from_strings(vec![
        "....",
        ".@..",
        ".E.."
    ]);

    game.input.simulate_move(Down);
    game.verify_next_tiles(vec![
        "....",
        ".@..",
        ".E.."
    ]);
}
