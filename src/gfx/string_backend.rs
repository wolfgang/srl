use crate::game::object_type::ObjectType::*;
use crate::game::object_type::ObjectType;

pub struct StringBackend {
    tiles: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl StringBackend {
    pub fn new(width: usize, height: usize) -> Self {
        let mut backend = Self {
            width,
            height,
            tiles: Vec::new(),
        };
        backend.clear();
        backend
    }

    pub fn tiles_as_string(&self) -> String {
        let strings: Vec<String> = self.tiles
            .iter()
            .map(|row| { row.iter().collect() })
            .collect();

        strings.join("\n")
    }

    pub fn tiles(&self) -> &Vec<Vec<char>> {
        &self.tiles
    }


    pub fn clear(&mut self) {
        self.tiles = vec![vec!['.'; self.width]; self.height];
    }

    pub fn render_at(&mut self, x: u32, y: u32, object_type: ObjectType) {
        let ch = match object_type {
            Wall => { '#' }
            Player => { '@' }
            Enemy => { 'E' }
            Floor => { '.' }
        };

        self.tiles[y as usize][x as usize] = ch;
    }
}
