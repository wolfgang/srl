use crate::_tests::helpers::rendering_spy::RenderingSpy;
use crate::game::Game;

#[test]
fn renders_walls_enemies_and_player() {
    let mut game = Game::new();
    game.add_enemies(&vec![(1, 1), (4, 2), (2, 3)]);
    game.add_walls(&vec![(1, 0), (2, 0), (3, 1), (1, 2)]);
    game.set_player_position(3, 2);
    let mut renderer = RenderingSpy::new(5, 4);
    game.render(&mut renderer);
    renderer.assert_frame(vec![
        ". # # . .",
        ". E . # .",
        ". # . @ E",
        ". . E . ."
    ])
}