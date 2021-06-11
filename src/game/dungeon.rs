use crate::game::object_type::ObjectType::{Enemy, Wall};
use crate::game::ObjectType;

pub type DungeonCoords = (u32, u32);
pub type DungeonObject = (DungeonCoords, ObjectType);

#[derive(Default)]
pub struct Dungeon {
    objects: Vec<DungeonObject>,
    player_position: DungeonCoords,
}

impl Dungeon {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            player_position: (0, 0),
        }
    }

    pub fn get_objects(&self) -> &Vec<DungeonObject> {
        &self.objects
    }

    pub fn get_player_position(&self) -> DungeonCoords {
        self.player_position
    }

    pub fn add_walls(&mut self, walls: &Vec<DungeonCoords>) {
        for pos in walls { self.objects.push((*pos, Wall))}
    }

    pub fn add_enemies(&mut self, enemies: &Vec<DungeonCoords>) {
        for pos in enemies { self.objects.push((*pos, Enemy))}
    }

    pub fn set_player_position(&mut self, x: u32, y: u32) {
        self.player_position = (x, y);
    }

    pub fn move_player_left(&mut self) {
        self.try_move_by(-1, 0);
    }

    pub fn move_player_right(&mut self) {
        self.try_move_by(1, 0);
    }

    pub fn move_player_up(&mut self) {
        self.try_move_by(0, -1);

    }

    pub fn move_player_down(&mut self) {
        self.try_move_by(0, 1);
    }

    fn try_move_by(&mut self, x_offset: i32, y_offset: i32 ) {
        let (player_x, player_y) = self.player_position;
        let new_player_position = ((player_x as i32 + x_offset) as u32, (player_y as i32 + y_offset) as u32);
        if !self.is_occupied(new_player_position) {
            self.player_position = new_player_position
        };
    }

    fn is_occupied(&self, coords: DungeonCoords) -> bool {
        self.objects.iter().any(|(obj_coords, _)| {*obj_coords == coords})
    }
}
