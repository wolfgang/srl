use std::collections::HashMap;

use crate::game::object_type::ObjectType::{Enemy, Wall};
use crate::game::ObjectType;
use crate::input::move_direction::MoveDirection;
use crate::input::move_direction::MoveDirection::*;

pub type DungeonCoords = (u32, u32);
pub type DungeonObjectTuple = (DungeonCoords, ObjectType);

pub type CollisionResult = Option<DungeonObjectTuple>;

struct Creature {
    hp: i32,
}

#[derive(Default)]
pub struct Dungeon {
    enemies: HashMap<DungeonCoords, Creature>,
    player_position: DungeonCoords,
    object_types: HashMap<DungeonCoords, ObjectType>,
}

const DEFAULT_ENEMY_HP: i32 = 100;

impl Dungeon {
    pub fn new() -> Self {
        Self {
            enemies: HashMap::new(),
            object_types: HashMap::new(),
            player_position: (0, 0),
        }
    }

    pub fn get_object_types(&self) -> Vec<DungeonObjectTuple> {
        self.object_types
            .iter()
            .map(|(coords, object_type)| { (*coords, *object_type) })
            .collect()
    }

    pub fn get_player_position(&self) -> DungeonCoords {
        self.player_position
    }

    pub fn add_walls(&mut self, walls: &Vec<DungeonCoords>) {
        for pos in walls {
            self.object_types.insert(*pos, Wall);
        }
    }

    pub fn add_enemies(&mut self, enemies: &Vec<DungeonCoords>) {
        for pos in enemies {
            self.enemies.insert(*pos, Creature { hp: DEFAULT_ENEMY_HP });
            self.object_types.insert(*pos, Enemy);
        }
    }

    pub fn remove_enemy(&mut self, x: u32, y: u32) {
        self.enemies.remove(&(x, y));
        self.object_types.remove(&(x, y));
    }

    pub fn apply_damage(&mut self, coords: DungeonCoords, damage: u32) -> i32 {
        let enemy = self.enemies.get_mut(&coords);
        debug_assert!(enemy.is_some(), "No enemy at {:?}", coords);
        let enemy = enemy.unwrap();
        enemy.hp -= damage as i32;
        enemy.hp
    }

    pub fn set_player_position(&mut self, x: u32, y: u32) {
        self.player_position = (x, y);
    }

    pub fn move_player(&mut self, direction: MoveDirection) -> CollisionResult {
        let offsets = match direction {
            Left => { (-1, 0) }
            Right => { (1, 0) }
            Up => { (0, -1) }
            Down => { (0, 1) }
        };
        self.try_move_by(offsets)
    }


    fn try_move_by(&mut self, (x_offset, y_offset): (i32, i32)) -> CollisionResult {
        let (player_x, player_y) = self.player_position;
        let new_player_position = ((player_x as i32 + x_offset) as u32, (player_y as i32 + y_offset) as u32);
        if let Some(object_type) = self.object_type_at(new_player_position) {
            Some((new_player_position, *object_type))
        } else {
            self.player_position = new_player_position;
            None
        }
    }

    fn object_type_at(&self, coords: DungeonCoords) -> Option<&ObjectType> {
        self.object_types.get(&coords)
    }
}
