use crate::game::dungeon::{CollisionResult, DungeonRef};
use crate::input::move_direction::MoveDirection;
use crate::input::move_direction::MoveDirection::*;

pub struct PlayerController {
    dungeon_ref: DungeonRef,
}

impl PlayerController {
    pub fn new(dungeon_ref: DungeonRef) -> Self {
        Self { dungeon_ref }
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
        let (player_x, player_y) = self.dungeon_ref.borrow().get_player_position();
        let new_player_position = (
            (player_x as i32 + x_offset) as u32,
            (player_y as i32 + y_offset) as u32
        );
        if let Some(object_type) = self.dungeon_ref.borrow().object_type_at(new_player_position) {
            return Some((new_player_position, *object_type));
        }
        self.dungeon_ref.borrow_mut().set_player_position(new_player_position.0, new_player_position.1);
        None
    }
}