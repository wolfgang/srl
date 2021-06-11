use crate::game::dungeon::Dungeon;
use crate::game::game_config::GameConfig;
use crate::game::object_type::ObjectType::{Enemy, Player, Wall};
use crate::gfx::renderer::Renderer;
use crate::input::Input;

pub struct Game {
    dungeon: Dungeon,
}

impl Game {
    pub fn new(_config: GameConfig) -> Self {
        Self {
            dungeon: Dungeon::new(),
        }
    }

    pub fn add_enemies(&mut self, positions: &Vec<(u32, u32)>) {
        self.dungeon.add_enemies(positions);
    }

    pub fn add_walls(&mut self, positions: &Vec<(u32, u32)>) {
        self.dungeon.add_walls(positions);
    }

    pub fn set_player_position(&mut self, x: u32, y: u32) {
        self.dungeon.set_player_position(x, y);
    }

    pub fn tick<T: Input>(&mut self, input: &T) {
        if input.move_left() { self.dungeon.move_player_left() }

        if input.move_right() { self.dungeon.move_player_right() }

        if input.move_up() { self.dungeon.move_player_up() }

        if input.move_down() { self.dungeon.move_player_down() }
    }


    pub fn render<T: Renderer>(&self, renderer: &mut T) {
        renderer.clear();
        for (x, y) in self.dungeon.get_enemies() {
            renderer.render_at(*x, *y, Enemy);
        }

        for (x, y) in self.dungeon.get_walls() {
            renderer.render_at(*x, *y, Wall);
        }

        let (player_x, player_y) = self.dungeon.get_player_position();
        renderer.render_at(player_x, player_y, Player);
    }
}