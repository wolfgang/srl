use crate::input::move_direction::MoveDirection;

pub trait Input {
    fn quit_game(&self) -> bool;
    fn wants_to_move(&self, direction: MoveDirection) -> bool;
}