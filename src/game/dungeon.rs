use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::game::creature::Creature;
use crate::game::object_type::ObjectType::{Enemy, Wall};
use crate::game::ObjectType;

pub type DungeonCoords = (u32, u32);
pub type DungeonObjectTuple = (DungeonCoords, ObjectType);

pub type CollisionResult = Option<DungeonObjectTuple>;
pub type DungeonRef = Rc<RefCell<Dungeon>>;

pub struct Dungeon {
    enemies: HashMap<DungeonCoords, Creature>,
    player: Creature,
    player_position: DungeonCoords,
    object_types: HashMap<DungeonCoords, ObjectType>,
}

const DEFAULT_PLAYER_HP: i32 = 100;

impl Dungeon {
    pub fn new() -> Self {
        Self {
            player: Creature { hp: DEFAULT_PLAYER_HP },
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

    pub fn add_enemy(&mut self, coords: DungeonCoords, enemy: Creature) {
        self.enemies.insert(coords, enemy);
        self.object_types.insert(coords, Enemy);

    }

    pub fn remove_enemy(&mut self, coords: DungeonCoords) {
        self.enemies.remove(&coords);
        self.object_types.remove(&coords);
    }

    pub fn damage_enemy(&mut self, coords: DungeonCoords, damage: u32) -> i32 {
        let enemy = self.enemies.get_mut(&coords);
        debug_assert!(enemy.is_some(), "No enemy at {:?}", coords);
        let enemy = enemy.unwrap();
        enemy.hp -= damage as i32;
        enemy.hp
    }

    pub fn damage_player(&mut self, damage: u32) -> i32 {
        self.player.hp -= damage as i32;
        self.player.hp
    }

    pub fn set_player_position(&mut self, x: u32, y: u32) {
        self.player_position = (x, y);
    }

    pub fn get_player_hp(&self) -> i32 {
        self.player.hp
    }

    pub fn get_num_enemies(&self) -> usize {
        return self.enemies.len()
    }

    pub(crate) fn get_object_type_at(&self, coords: DungeonCoords) -> Option<&ObjectType> {
        self.object_types.get(&coords)
    }
}
