use crate::game::dungeon::Dungeon;
use crate::game::game_config::GameConfig;
use crate::game::object_type::ObjectType::{Enemy, Player, Wall};
use crate::gfx::renderer::Renderer;
use crate::input::Input;

type CellCoords = (u32, u32);

pub struct Game {
    player: CellCoords,
    dungeon: Dungeon
}

impl Game {
    pub fn new(_config: GameConfig) -> Self {
        Self {
            dungeon: Dungeon::new(),
            player: (0, 0),
        }
    }

    pub fn add_enemies(&mut self, positions: &Vec<(u32, u32)>) {
        self.dungeon.add_enemies(positions);
    }

    pub fn add_walls(&mut self, positions: &Vec<(u32, u32)>) {
        self.dungeon.add_walls(positions);
    }

    pub fn set_player_position(&mut self, x: u32, y: u32) {
        self.player = (x, y);
    }

    pub fn tick<T: Input>(&mut self, input: &T) {
        if input.move_left() && self.can_move_to(-1, 0) {
            self.player.0 -= 1;
        }

        if input.move_right() && self.can_move_to(1, 0) {
            self.player.0 += 1
        }

        if input.move_up() && self.can_move_to(0, -1) {
            self.player.1 -= 1
        }

        if input.move_down() && self.can_move_to(0, 1) {
            self.player.1 += 1
        }
    }


    pub fn render<T: Renderer>(&self, renderer: &mut T) {
        renderer.clear();
        for (x, y) in self.dungeon.get_enemies() {
            renderer.render_at(*x, *y, Enemy);
        }

        for (x, y) in self.dungeon.get_walls() {
            renderer.render_at(*x, *y, Wall);
        }

        renderer.render_at(self.player.0, self.player.1, Player);
    }

    fn can_move_to(&self, x_offset: i32, y_offset: i32) -> bool {
        let new_pos = ((self.player.0 as i32 + x_offset) as u32, (self.player.1 as i32 + y_offset) as u32);

        !self.dungeon.get_walls().contains(&new_pos) && !self.dungeon.get_enemies().contains(&new_pos)
    }
}