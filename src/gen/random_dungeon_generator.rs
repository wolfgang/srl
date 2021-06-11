use crate::game::dungeon::Dungeon;
use crate::gen::dungeon_generator::DungeonGenerator;

pub struct RandomDungeonGenerator {}

impl RandomDungeonGenerator {
    pub fn new(num_walls: u32, num_enemies: u32) -> Self {
        Self {}
    }
}

impl DungeonGenerator for RandomDungeonGenerator {
    fn generate(&self) -> Dungeon {
        let mut dungeon = Dungeon::new();
        dungeon.add_walls(&vec![(1, 2), (2, 3), (4, 5)]);
        dungeon.add_enemies(&vec![(6, 7), (8, 9)]);
        dungeon
    }
}