use crate::_tests::helpers::fixed_dungeon_generator::FixedDungeonGenerator;
use crate::_tests::helpers::rendering_spy::RenderingSpy;
use crate::game::Game;

#[test]
fn generates_game_using_given_generator() {
    let mut generator = FixedDungeonGenerator::new();
    generator.generate_walls(vec!((0, 0), (1, 0), (2, 0)));
    generator.generate_enemies(vec!((1, 1), (2, 1)));
    generator.generate_player(0, 1);

    let game = Game::generate_with(&generator);
    let mut renderer = RenderingSpy::new(4, 2);
    game.render(&mut renderer);
    renderer.assert_frame(vec![
        "# # # .",
        "@ E E ."
    ])
}