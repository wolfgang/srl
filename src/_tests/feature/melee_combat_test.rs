// use crate::_tests::helpers::testable_game::TestableGame;
// use crate::game::GameConfig;
//
// #[test]
// fn combat_on_collision() {
//     let combat_engine = ControlledCombatEngine::new();
//     combat_engine.say_is_hit((1, 0), (2, 0));
//     combat_engine.say_is_hit((2, 0), (1, 0));
//     combat_engine.say_damage((1, 0), 6);
//     combat_engine.say_damage((2, 0), 2);
//
//     let mut game = TestableGame::from_strings(vec![".@E."], combat_engine);
//
//     game.input.simulate_move_right();
//     game.tick();
//     game.renderer.assert_combat_log(vec![
//         "Player hits Enemy for 6 damage!",
//         "Enemy hits Player for 2 damage!"
//     ])
// }