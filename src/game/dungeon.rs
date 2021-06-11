
pub struct Dungeon {
    walls: Vec<(u32, u32)>,
    enemies: Vec<(u32, u32)>,
}

impl Dungeon {
    pub fn new() -> Self {
        Self {walls: Vec::new(), enemies: Vec::new()}
    }

    pub fn get_walls(&self) -> &Vec<(u32, u32)> {
        &self.walls
    }

    pub fn get_enemies(&self) -> &Vec<(u32, u32)> {
        &self.enemies
    }

}