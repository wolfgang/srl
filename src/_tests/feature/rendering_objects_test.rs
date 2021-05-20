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
    game.add_enemies(&vec![(1, 1), (4, 2), (2, 3)]);
    game.add_walls(&vec![(1, 0), (2, 0), (3, 1), (1, 2)]);
    game.set_player_position(3, 2);
    let mut renderer = RenderingSpy::new(4, 5);
    game.render(&mut renderer);
    renderer.assert_frame(vec![
        ". # # . .",
        ". e . # .",
        ". # . @ e",
        ". . e . ."
    ])
}