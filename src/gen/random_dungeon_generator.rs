use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::game::creature::Creature;
use crate::game::dungeon::Dungeon;
use crate::gen::dungeon_generator::DungeonGenerator;

pub struct RandomDungeonGenerator {
    num_walls: usize,
    num_enemies: usize,
    width: usize,
    height: usize
}

impl RandomDungeonGenerator {
    pub fn new(width: usize, height: usize, num_walls: usize, num_enemies: usize) -> Self {
        Self {width, height, num_walls, num_enemies}
    }
}

impl DungeonGenerator for RandomDungeonGenerator {
    fn generate(&self) -> Dungeon {
        let mut available_coords = Vec::with_capacity((self.width*self.height) as usize);

        for y in 0 .. self.height {
            for x in 0 ..self.width {
                available_coords.push((x as u32, y as u32));
            }
        }
        available_coords.shuffle(&mut thread_rng());

        let mut dungeon = Dungeon::new();
        dungeon.add_walls(&available_coords[0..self.num_walls].to_vec());

        for coord in &available_coords[self.num_walls .. self.num_walls  + self.num_enemies] {
            dungeon.add_enemy(*coord, Creature {hp: 20})
        }

        let player_position = available_coords[self.num_walls + self.num_enemies];
        dungeon.set_player_position(player_position.0, player_position.1);
        dungeon
    }
}