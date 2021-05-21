use crate::_tests::helpers::input_simulator::InputSimulator;
use crate::_tests::helpers::rendering_spy::RenderingSpy;
use crate::game::Game;
use crate::game_config::GameConfig;

pub struct TestableGame {
    game: Game,
    pub renderer: RenderingSpy,
    pub input: InputSimulator,
}

impl TestableGame {
    pub fn new(config: GameConfig) -> Self {
        let (width, height) = config.dungeon_size;
        Self {
            game: Game::new(config),
            renderer: RenderingSpy::new(width, height),
            input: InputSimulator::new(),
        }
    }

    pub fn tick(&mut self) {
        self.game.tick(&mut self.input)
    }

    pub fn render(&mut self) {
        self.game.render(&mut self.renderer);
    }

    pub fn set_player_position(&mut self, x: u32, y: u32) {
        self.game.set_player_position(x, y);
    }

    pub fn verify_next_frame(&mut self, expected: Vec<&str>) {
        self.tick();
        self.render();
        self.renderer.assert_frame(expected);
    }
}
