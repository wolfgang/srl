use crate::_tests::helpers::testable_game::TestableGame;

#[test]
fn player_collides_with_enemy_to_the_right() {
    let mut game = TestableGame::from_strings(vec![
        "....",
        ".@E.",
        "...."
    ]);

    game.input.simulate_move_right();
    game.verify_next_frame(vec![
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

    game.input.simulate_move_left();
    game.verify_next_frame(vec![
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

    game.input.simulate_move_up();
    game.verify_next_frame(vec![
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

    game.input.simulate_move_down();
    game.verify_next_frame(vec![
        "....",
        ".@..",
        ".E.."
    ]);
}
