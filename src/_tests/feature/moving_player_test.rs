use crate::_tests::helpers::rendering_spy::RenderingSpy;
use crate::game::Game;
use crate::game_config::GameConfig;

#[test]
fn player_moves_according_to_input() {
    let config = GameConfig {
        dungeon_size: (5, 4)
    };

    let mut game = Game::new(config);
    game.set_player_position(2, 1);
    let mut renderer = RenderingSpy::new(5, 4);
    game.render(&mut renderer);
    renderer.assert_frame(vec![
        ". . . . .",
        ". . @ . .",
        ". . . . .",
        ". . . . ."
    ])
}