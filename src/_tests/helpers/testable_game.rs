use crate::_tests::helpers::input_simulator::InputSimulator;
use crate::_tests::helpers::rendering_spy::RenderingSpy;
use crate::game::Game;
use crate::game::game_config::GameConfig;

pub struct TestableGame {
    game: Game,
    pub renderer: RenderingSpy,
    pub input: InputSimulator,
}

impl TestableGame {
    pub fn from_strings(strings: Vec<&str>) -> Self {
        let strings: Vec<String> = strings.iter().map(|str| { str.replace(" ", "") }).collect();
        let width = strings[0].len();
        let height = strings.len();
        let config = GameConfig { dungeon_size: (width, height) };
        let (width, height) = config.dungeon_size;
        let mut game = Self {
            game: Game::new(config),
            renderer: RenderingSpy::new(width, height),
            input: InputSimulator::new(),
        };
        let mut walls: Vec<(u32, u32)> = Vec::new();
        let mut enemies: Vec<(u32, u32)> = Vec::new();
        for (y, row) in strings.iter().enumerate() {
            let chars = row.chars().collect::<Vec<char>>();
            for (x, current) in chars.iter().enumerate() {
                let pos = (x as u32, y as u32);
                match current {
                    '#' => { walls.push(pos) }
                    'E' => { enemies.push(pos) }
                    '@' => { game.set_player_position(pos.0, pos.1) }
                    _ => {}
                }
            }
        }
        game.game.add_walls(&walls);
        game.game.add_enemies(&enemies);
        game
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
