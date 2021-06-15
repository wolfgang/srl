use crate::input::move_direction::MoveDirection;

pub trait Input {
    fn move_left(&self) -> bool;
    fn move_right(&self) -> bool;
    fn move_up(&self) -> bool;
    fn move_down(&self) -> bool;
    fn quit_game(&self) -> bool;

    fn wants_to_move(&self, direction: MoveDirection) -> bool;
}