use crate::_tests::helpers::rendering_spy::RenderingSpy;
use crate::game::Game;
use crate::game_config::GameConfig;

#[ignore]
#[test]
fn renders_walls_enemies_and_player() {
    let config = GameConfig {
        dungeon_size: (4, 5)
    };

    let mut game = Game::new(config);
    game.add_enemies(&vec![(1, 1), (2, 4), (3, 2)]);
    game.add_walls(&vec![(0, 1), (0, 2), (1, 3), (2, 1)]);
    game.set_player_position(2, 3);
    let mut renderer = RenderingSpy::new(4, 5);
    game.render(&mut renderer);
    renderer.assert_frame(vec![
        ". # # . .",
        ". e . # .",
        ". # . @ e",
        ". . e . ."
    ])
}