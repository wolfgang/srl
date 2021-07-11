use crate::_tests::_helpers::testable_game::TestableGame;
use crate::input::move_direction::MoveDirection::Right;

#[test]
fn enemy_is_removed_after_one_hit() {
    let mut game = TestableGame::from_strings(vec![". @ E ."]);
    game.configure_combat(|combat_engine| {
        combat_engine.say_is_hit((1, 0), (2, 0));
        combat_engine.say_damage((1, 0), 1000);
    });

    game.input.simulate_move(Right);
    game.verify_next_tiles(vec![". . @ ."]);
}

#[test]
fn enemy_is_removed_after_two_hits() {
    let mut game = TestableGame::from_strings(vec![". @ E ."]);
    game.configure_combat(|combat_engine| {
        combat_engine.say_is_hit((1, 0), (2, 0));
        combat_engine.say_damage((1, 0), 60);
    });
    game.input.simulate_move(Right);
    game.verify_next_tiles(vec![". @ E ."]);
    game.verify_next_tiles(vec![". . @ ."]);
}

#[test]
fn combat_log_reflects_enemy_death() {
    let mut game = TestableGame::from_strings(vec![". @ E ."]);
    game.configure_combat(|combat_engine| {
        combat_engine.say_is_hit((1, 0), (2, 0));
        combat_engine.say_damage((1, 0), 1000);
    });

    game.input.simulate_move(Right);
    game.verify_next_combat_log(vec![
        "Player hits Enemy for 1000 damage!",
        "Enemy dies!"
    ])
}

