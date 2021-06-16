use crate::_tests::_helpers::fixed_dungeon_generator::FixedDungeonGenerator;
use crate::_tests::_helpers::input_simulator::InputSimulator;
use crate::_tests::_helpers::rendering_spy::RenderingSpy;
use crate::game::Game;

pub struct TestableGame {
    pub game: Game,
    pub renderer: RenderingSpy,
    pub input: InputSimulator,
}

impl TestableGame {
    pub fn from_strings(strings: Vec<&str>) -> Self {
        let strings: Vec<String> = strings.iter().map(|str| { str.replace(" ", "") }).collect();
        let width = strings[0].len();
        let height = strings.len();
        let mut walls: Vec<(u32, u32)> = Vec::new();
        let mut enemies: Vec<(u32, u32)> = Vec::new();
        let mut player_position = (0, 0);
        for (y, row) in strings.iter().enumerate() {
            let chars = row.chars().collect::<Vec<char>>();
            for (x, current) in chars.iter().enumerate() {
                let pos = (x as u32, y as u32);
                match current {
                    '#' => { walls.push(pos) }
                    'E' => { enemies.push(pos) }
                    '@' => { player_position = pos }
                    '.' => {}
                    c => {
                        assert!(false, "Invalid character in dungeon definition: {}", c);
                    }
                }
            }
        }

        let mut generator = FixedDungeonGenerator::new();
        generator.generate_walls(walls);
        generator.generate_enemies(enemies);
        generator.generate_player(player_position.0, player_position.1);

        Self {
            game: Game::generate_with(&generator),
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

    pub fn verify_next_tiles(&mut self, expected: Vec<&str>) {
        self.tick();
        self.render();
        self.renderer.assert_tiles(expected);
    }
}
