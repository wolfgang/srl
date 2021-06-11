use crate::game::dungeon::CellCoords;
use crate::gen::dungeon_generator::DungeonGenerator;

pub struct FixedDungeonGenerator {

}

impl FixedDungeonGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_walls(&mut self, walls: Vec<CellCoords>) {

    }

    pub fn generate_enemies(&mut self, walls: Vec<CellCoords>) {

    }

    pub fn generate_player(&mut self, x: u32, y: u32) {

    }
}

impl DungeonGenerator for FixedDungeonGenerator {

}