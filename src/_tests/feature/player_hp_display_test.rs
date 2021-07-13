use crate::_tests::_helpers::testable_game::TestableGame;
use crate::input::move_direction::MoveDirection::Left;

#[test]
fn displays_initial_player_hp_in_status() {
    let mut game = TestableGame::from_strings(vec![". @ . ."]);
    game.render();
    game.renderer.assert_player_hp_rendered(100);
}

#[test]
fn displays_modified_player_hp_in_status() {
    let mut game = TestableGame::from_strings(vec!["E @ . ."]);
    game.configure_combat(|combat_engine| {
        combat_engine.say_is_hit((0, 0), (1, 0));
        combat_engine.say_damage((0, 0), 25);
    });
    game.input.simulate_move(Left);
    game.tick();
    game.render();
    game.renderer.assert_player_hp_rendered(75);
}