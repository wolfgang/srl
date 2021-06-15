use crate::_tests::_helpers::controlled_combat_engine::ControlledCombatEngine;
use crate::_tests::_helpers::testable_game::TestableGame;

#[test]
fn combat_on_collision() {
    let mut combat_engine = ControlledCombatEngine::new();
    combat_engine.say_is_hit((1, 0), (2, 0));
    combat_engine.say_is_hit((2, 0), (1, 0));
    combat_engine.say_damage((1, 0), 6);
    combat_engine.say_damage((2, 0), 2);

    let mut game = TestableGame::from_strings(vec![".@E."]);
    game.game.override_combat_engine(combat_engine);

    game.input.simulate_move_right();
    game.tick();
    game.render();
    game.renderer.assert_combat_log(vec![
        "Player hits Enemy for 6 damage!",
        "Enemy hits Player for 2 damage!"
    ])
}