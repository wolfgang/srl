use crate::_tests::_helpers::testable_game::TestableGame;
use crate::game::game::GameState::{AllEnemiesDied, Running};
use crate::input::move_direction::MoveDirection::Right;

#[test]
fn game_state_changes_after_all_enemies_are_dead() {
    let mut game = TestableGame::from_strings(vec![" . @ E E"]);
    game.configure_combat(|combat_engine| {
        combat_engine.say_is_hit((1, 0), (2, 0));
        combat_engine.say_is_hit((2, 0), (3, 0));
        combat_engine.say_damage((1, 0), TestableGame::default_enemy_hp()*10);
        combat_engine.say_damage((2, 0), TestableGame::default_enemy_hp()*10);
    });
    game.input.simulate_move(Right);
    game.verify_next_tiles(vec![" . . @ E"]);
    assert_eq!(Running, game.game.game_state());
    game.verify_next_tiles(vec![" . . . @"]);
    assert_eq!(AllEnemiesDied, game.game.game_state());
}