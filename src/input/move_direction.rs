use crate::input::move_direction::MoveDirection::*;

#[derive(PartialEq, Copy, Clone)]
pub enum MoveDirection {
    Left,
    Right,
    Up,
    Down
}

impl MoveDirection {
    pub fn iter() -> std::slice::Iter<'static, MoveDirection> {
        static DIRECTIONS: [MoveDirection; 4] = [Left, Right, Up, Down];
        DIRECTIONS.iter()
    }
}
