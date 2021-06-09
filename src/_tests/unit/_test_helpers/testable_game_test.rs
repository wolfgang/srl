use crate::_tests::helpers::testable_game::TestableGame;

#[test]
fn can_be_constructed_from_strings() {
    let mut game = TestableGame::from_strings(vec![
        "#..#",
        "E.@#",
        "####"
    ]);

    game.render();
    game.renderer.assert_frame(vec![
        "#..#",
        "E.@#",
        "####"
    ]);
}