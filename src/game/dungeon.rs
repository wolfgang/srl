
pub struct Dungeon {
    walls: Vec<(u32, u32)>,
    enemies: Vec<(u32, u32)>,
    player_position: (u32, u32)
}

impl Dungeon {
    pub fn new() -> Self {
        Self {
            walls: Vec::new(),
            enemies: Vec::new(),
            player_position: (0, 0)
        }
    }

    pub fn get_walls(&self) -> &Vec<(u32, u32)> {
        &self.walls
    }

    pub fn get_enemies(&self) -> &Vec<(u32, u32)> {
        &self.enemies
    }

    pub fn get_player_position(&self) -> (u32, u32) {
        self.player_position
    }

    pub fn add_walls(&mut self, walls: &Vec<(u32, u32)>) {
        self.walls = walls.clone();
    }

    pub fn add_enemies(&mut self, enemies: &Vec<(u32, u32)>) {
        self.enemies = enemies.clone();
    }

    pub fn set_player_position(&mut self, x: u32, y: u32) {
        self.player_position = (x, y);
    }

    pub fn move_player_left(&mut self) {
        self.player_position.0 -= 1;
    }

    pub fn move_player_right(&mut self) {
        self.player_position.0 += 1;

    }

    pub fn move_player_up(&mut self) {
        self.player_position.1 -= 1;

    }

    pub fn move_player_down(&mut self) {
        self.player_position.1 += 1;
    }

}