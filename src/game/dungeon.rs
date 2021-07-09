use crate::game::object_type::ObjectType::{Enemy, Floor, Wall};
use crate::game::ObjectType;
use crate::input::move_direction::MoveDirection;
use crate::input::move_direction::MoveDirection::*;

pub type DungeonCoords = (u32, u32);
pub type DungeonObjectTuple = (DungeonCoords, ObjectType);

pub type CollisionResult = Option<DungeonObjectTuple>;


#[derive(Default)]
pub struct Dungeon {
    objects: Vec<DungeonObjectTuple>,
    player_position: DungeonCoords,
}

impl Dungeon {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            player_position: (0, 0),
        }
    }

    pub fn get_objects(&self) -> &Vec<DungeonObjectTuple> {
        &self.objects
    }

    pub fn get_player_position(&self) -> DungeonCoords {
        self.player_position
    }

    pub fn add_walls(&mut self, walls: &Vec<DungeonCoords>) {
        for pos in walls { self.objects.push((*pos, Wall)) }
    }

    pub fn add_enemies(&mut self, enemies: &Vec<DungeonCoords>) {
        for pos in enemies { self.objects.push((*pos, Enemy)) }
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
        let object_type = self.object_type_at(new_player_position);
        if object_type == Floor {
            self.player_position = new_player_position;
            return None;
        }
        Some((new_player_position, object_type))
    }

    fn object_type_at(&self, coords: DungeonCoords) -> ObjectType {
        match self.objects.iter().find(|(obj_coords, _)| { *obj_coords == coords }) {
            Some((_, object_type)) => { *object_type }
            None => Floor
        }
    }
}
