use crate::game::creature::Creature;
use crate::game::dungeon::{Dungeon, DungeonCoords};
use crate::gen::dungeon_generator::DungeonGenerator;

#[derive(Default)]
pub struct FixedDungeonGenerator {
    walls: Vec<DungeonCoords>,
    enemies: Vec<DungeonCoords>,
    player: DungeonCoords,
    enemies_hp: u32
}

impl FixedDungeonGenerator {
    pub fn new() -> Self {
        Self {
            enemies_hp: 100,
            .. Default::default()
        }
    }

    pub fn generate_walls(&mut self, walls: Vec<DungeonCoords>) {
        self.walls = walls;
    }

    pub fn generate_enemies(&mut self, enemies: Vec<DungeonCoords>) {
        self.enemies = enemies;

    }

    pub fn generate_player(&mut self, x: u32, y: u32) {
        self.player = (x, y);
    }

    pub fn roll_enemies_hp(&mut self, value: u32) {
        self.enemies_hp = value;
    }
}

impl DungeonGenerator for FixedDungeonGenerator {
    fn generate(&self) -> Dungeon {
        let mut dungeon = Dungeon::new();
        dungeon.add_walls(&self.walls);
        for pos in &self.enemies {
            dungeon.add_enemy(*pos, Creature {hp: self.enemies_hp as i32});
        }
        dungeon.set_player_position(self.player.0, self.player.1);
        dungeon
    }
}