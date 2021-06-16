use crate::game::object_type::ObjectType::*;
use crate::game::object_type::ObjectType;

pub struct StringBackend {
    tiles: Vec<Vec<char>>,
    combat_log: Vec<String>,
    width: usize,
    height: usize,
}

impl StringBackend {
    pub fn new(width: usize, height: usize) -> Self {
        let mut backend = Self {
            width,
            height,
            tiles: Vec::new(),
            combat_log: Vec::new()
        };
        backend.clear();
        backend
    }

    pub fn tiles_as_string(&self) -> String {
        self.tiles_as_strings() .join("\n")
    }

    pub fn tiles_as_strings(&self) -> Vec<String> {
       self.tiles
            .iter()
            .map(|row| { row.iter().collect() })
            .collect()
    }

    pub fn tiles(&self) -> &Vec<Vec<char>> {
        &self.tiles
    }

    pub fn combat_log(&self) -> Vec<&str> {
        self.combat_log.iter().map(|s| {s.as_str()}).collect()
    }


    pub fn clear(&mut self) {
        self.tiles = vec![vec!['.'; self.width]; self.height];
        self.combat_log.clear();
    }

    pub fn render_tile(&mut self, x: u32, y: u32, object_type: ObjectType) {
        let ch = match object_type {
            Wall => { '#' }
            Player => { '@' }
            Enemy => { 'E' }
            Floor => { '.' }
        };

        self.tiles[y as usize][x as usize] = ch;
    }

    pub fn append_combat_log(&mut self, text: &str) {
        self.combat_log.push(text.to_string());

    }
}
