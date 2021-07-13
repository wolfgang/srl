use crate::_tests::_helpers::testable_game::TestableGame;

#[test]
fn displays_player_hp_in_status() {
    let mut game = TestableGame::from_strings(vec![". @ . ."]);
    game.render();
    game.renderer.assert_player_hp_rendered(100);



}