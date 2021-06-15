use crate::_tests::_helpers::testable_game::TestableGame;

#[test]
fn player_collides_with_north_wall() {
    let mut game = TestableGame::from_strings(vec![
        "###",
        ".@.",
    ]);

    game.input.simulate_move_up();
    game.verify_next_tiles(vec![
        "###",
        ".@.",
    ]);
}



#[test]
fn player_collides_with_west_wall() {
    let mut game = TestableGame::from_strings(vec![
        "#.",
        "#@",
        "#.",
    ]);

    game.input.simulate_move_left();
    game.verify_next_tiles(vec![
        "#.",
        "#@",
        "#.",
    ]);
}

#[test]
fn player_collides_with_east_wall() {
    let mut game = TestableGame::from_strings(vec![
        "..#",
        ".@#",
        "..#",
    ]);

    game.input.simulate_move_right();
    game.verify_next_tiles(vec![
        "..#",
        ".@#",
        "..#",
    ]);
}