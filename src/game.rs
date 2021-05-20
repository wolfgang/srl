use crate::game_config::GameConfig;
use crate::renderer::Renderer;

pub struct Game {}

impl Game {
    pub fn new(_config: GameConfig) -> Self {
        Self {}
    }

    pub fn add_enemies(&mut self, _positions: &Vec<(u32, u32)>) {}

    pub fn add_walls(&mut self, _positions: &Vec<(u32, u32)>) {}

    pub fn set_player_position(&mut self, _x: u32, _y: u32) {}

    pub fn render<T: Renderer>(&self, _renderer: &mut T) {}
}