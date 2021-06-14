#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum ObjectType {
    Wall,
    Player,
    Enemy,
    Floor
}
