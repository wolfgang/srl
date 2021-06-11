use crate::game::object_type::ObjectType::{Enemy, Wall};
use crate::game::ObjectType;

pub type CellCoords = (u32, u32);
pub type DungeonObject = (CellCoords, ObjectType);

pub struct Dungeon {
    objects: Vec<DungeonObject>,
    player_position: CellCoords,
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

    pub fn get_player_position(&self) -> CellCoords {
        self.player_position
    }

    pub fn add_walls(&mut self, walls: &Vec<CellCoords>) {
        for pos in walls { self.objects.push((*pos, Wall))}
    }

    pub fn add_enemies(&mut self, enemies: &Vec<CellCoords>) {
        for pos in enemies { self.objects.push((*pos, Enemy))}
    }

    pub fn set_player_position(&mut self, x: u32, y: u32) {
        self.player_position = (x, y);
    }

    pub fn move_player_left(&mut self) {
        if self.can_player_move_to(-1, 0) { self.player_position.0 -= 1 };
    }

    pub fn move_player_right(&mut self) {
        if self.can_player_move_to(1, 0) { self.player_position.0 += 1 }
    }

    pub fn move_player_up(&mut self) {
        if self.can_player_move_to(0, -1) { self.player_position.1 -= 1 };
    }

    pub fn move_player_down(&mut self) {
        if self.can_player_move_to(0, 1) { self.player_position.1 += 1 };
    }

    fn can_player_move_to(&self, x_offset: i32, y_offset: i32) -> bool {
        let (player_x, player_y) = self.player_position;
        let new_pos = ((player_x as i32 + x_offset) as u32, (player_y as i32 + y_offset) as u32);

        !self.objects.iter().any(|(pos, _)| {*pos == new_pos})
    }
}
