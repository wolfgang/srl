use crate::game_config::GameConfig;
use crate::object_type::ObjectType::{Enemy, Player, Wall};
use crate::renderer::Renderer;

type CellCoords = (u32, u32);

pub struct Game {
    enemies: Vec<CellCoords>,
    walls: Vec<CellCoords>,
    player: CellCoords,
}

impl Game {
    pub fn new(_config: GameConfig) -> Self {
        Self {
            enemies: Vec::with_capacity(64),
            walls: Vec::with_capacity(64),
            player: (0, 0),
        }
    }

    pub fn add_enemies(&mut self, positions: &Vec<(u32, u32)>) {
        self.enemies = positions.clone();
    }

    pub fn add_walls(&mut self, positions: &Vec<(u32, u32)>) {
        self.walls = positions.clone();
    }

    pub fn set_player_position(&mut self, x: u32, y: u32) {
        self.player = (x, y);
    }

    pub fn render<T: Renderer>(&self, renderer: &mut T) {
        for (x, y) in &self.enemies {
            renderer.render_at(*x, *y, Enemy);
        }

        for (x, y) in &self.walls {
            renderer.render_at(*x, *y, Wall);
        }

        renderer.render_at(self.player.0, self.player.1, Player);
    }
}