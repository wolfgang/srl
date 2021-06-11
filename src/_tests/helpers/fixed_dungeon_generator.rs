use crate::game::dungeon::{CellCoords, Dungeon};
use crate::gen::dungeon_generator::DungeonGenerator;

#[derive(Default)]
pub struct FixedDungeonGenerator {
    walls: Vec<CellCoords>,
    enemies: Vec<CellCoords>,
    player: CellCoords
}

impl FixedDungeonGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate_walls(&mut self, walls: Vec<CellCoords>) {
        self.walls = walls;
    }

    pub fn generate_enemies(&mut self, enemies: Vec<CellCoords>) {
        self.enemies = enemies;

    }

    pub fn generate_player(&mut self, x: u32, y: u32) {
        self.player = (x, y);
    }
}

impl DungeonGenerator for FixedDungeonGenerator {
    fn generate(&self) -> Dungeon {
        let mut dungeon = Dungeon::new();
        dungeon.add_walls(&self.walls);
        dungeon.add_enemies(&self.enemies);
        dungeon.set_player_position(self.player.0, self.player.1);
        dungeon
    }
}