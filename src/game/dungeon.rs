pub struct Dungeon {
    walls: Vec<(u32, u32)>,
    enemies: Vec<(u32, u32)>,
    player_position: (u32, u32),
}

impl Dungeon {
    pub fn new() -> Self {
        Self {
            walls: Vec::new(),
            enemies: Vec::new(),
            player_position: (0, 0),
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
        if self.can_move_to(-1, 0) { self.player_position.0 -= 1 };
    }

    pub fn move_player_right(&mut self) {
        if self.can_move_to(1, 0) { self.player_position.0 += 1 }
    }

    pub fn move_player_up(&mut self) {
        if self.can_move_to(0, -1) { self.player_position.1 -= 1 };
    }

    pub fn move_player_down(&mut self) {
        if self.can_move_to(0, 1) { self.player_position.1 += 1 };
    }

    fn can_move_to(&self, x_offset: i32, y_offset: i32) -> bool {
        let (player_x, player_y) = self.player_position;
        let new_pos = ((player_x as i32 + x_offset) as u32, (player_y as i32 + y_offset) as u32);

        !self.walls.contains(&new_pos) && !self.enemies.contains(&new_pos)
    }
}
