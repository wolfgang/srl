pub trait Input {
    fn move_left(&self) -> bool;
    fn move_right(&self) -> bool;
    fn move_up(&self) -> bool;
    fn move_down(&self) -> bool;
}