use crate::_tests::_helpers::controlled_combat_engine::ControlledCombatEngine;
use crate::_tests::_helpers::testable_game::TestableGame;
use crate::input::move_direction::MoveDirection::Right;

#[test]
fn enemy_is_removed_when_dead() {
    let mut game = TestableGame::from_strings(vec![". @ E ."]);
    let mut combat_engine = ControlledCombatEngine::new();
    combat_engine.say_is_hit((1, 0), (2, 0));
    combat_engine.say_damage((1, 0), 1000);

    game.game.override_combat_engine(combat_engine);
    game.input.simulate_move(Right);
    game.verify_next_tiles(vec![". . @ ."]);
}