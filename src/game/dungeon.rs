use crate::game::object_type::ObjectType::{Enemy, Wall};
use crate::game::ObjectType;
use crate::input::move_direction::MoveDirection;
use crate::input::move_direction::MoveDirection::*;

pub type DungeonCoords = (u32, u32);
pub type DungeonObjectTuple = (DungeonCoords, ObjectType);

pub type CollisionResult = Option<DungeonObjectTuple>;

struct Creature {
    position: DungeonCoords,
    hp: i32
}

#[derive(Default)]
pub struct Dungeon {
    walls: Vec<DungeonCoords>,
    enemies: Vec<Creature>,
    player_position: DungeonCoords,
}

const DEFAULT_ENEMY_HP: i32 = 100;

impl Dungeon {
    pub fn new() -> Self {
        Self {
            walls: Vec::new(),
            enemies: Vec::new(),
            player_position: (0, 0),
        }
    }

    pub fn get_objects(&self) -> Vec<DungeonObjectTuple> {
        let mut result: Vec<DungeonObjectTuple> = self.walls.iter()
            .map(|coord| { (*coord, Wall) })
            .collect();

        for enemy in self.enemies.iter() {
            result.push((enemy.position, Enemy));
        }
        result
    }

    pub fn get_player_position(&self) -> DungeonCoords {
        self.player_position
    }

    pub fn add_walls(&mut self, walls: &Vec<DungeonCoords>) {
        for pos in walls { self.walls.push(*pos) }
    }

    pub fn add_enemies(&mut self, enemies: &Vec<DungeonCoords>) {
        for pos in enemies {
            self.enemies.push(Creature { position: *pos, hp: DEFAULT_ENEMY_HP });
        }
    }

    pub fn remove_enemy(&mut self, x: u32, y: u32) {
        let mut i = 0;
        while i < self.enemies.len() {
            if self.enemies[i].position == (x, y) {
                self.enemies.remove(i);
            }
            else {
                i += 1;
            }
        }
    }

    pub fn apply_damage(&mut self, coords: DungeonCoords, damage: u32) -> i32 {
        for mut enemy in &mut self.enemies {
            if enemy.position == coords {
                enemy.hp -= damage as i32;
                return enemy.hp;
            }
        }
        100
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
            Some((new_player_position, object_type))
        } else {
            self.player_position = new_player_position;
            None
        }
    }

    fn object_type_at(&self, coords: DungeonCoords) -> Option<ObjectType> {
        match self.get_objects().iter().find(|(obj_coords, _)| { *obj_coords == coords }) {
            Some((_, object_type)) => { Some(*object_type) }
            None => None
        }
    }
}
