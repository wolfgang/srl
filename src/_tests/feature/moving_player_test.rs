// use crate::_tests::helpers::rendering_spy::RenderingSpy;
// use crate::game::Game;
// use crate::game_config::GameConfig;
//
// #[test]
// fn player_moves_according_to_input() {
//     let config = GameConfig {
//         dungeon_size: (5, 4)
//     };
//
//     let mut game = Game::new(config);
//     game.set_player_position(2, 1);
//     let input = input_simulator::new();
//     let mut renderer = RenderingSpy::new(5, 4);
//     game.render(&mut renderer);
//     renderer.assert_frame(vec![
//         ". . . . .",
//         ". . @ . .",
//         ". . . . .",
//         ". . . . ."
//     ]);
//
//
//     input.simulate_move_right();
//     game.tick(&input);
//     renderer.assert_frame(vec![
//         ". . . . .",
//         ". . . @ .",
//         ". . . . .",
//         ". . . . ."
//     ]);
//
// }