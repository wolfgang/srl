use crate::game::dungeon::Dungeon;

pub trait DungeonGenerator {
    fn generate(&self) -> Dungeon;

}