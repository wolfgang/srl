use crate::game::object_type::ObjectType::*;
use crate::game::object_type::ObjectType;

pub struct StringBackend {
    frame: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl StringBackend {
    pub fn new(width: usize, height: usize) -> Self {
        let mut backend = Self {
            width,
            height,
            frame: Vec::new(),
        };
        backend.clear();
        backend
    }

    pub fn frame_as_string(&self) -> String {
        let strings: Vec<String> = self.frame
            .iter()
            .map(|row| { row.iter().collect() })
            .collect();

        strings.join("\n")
    }

    pub fn frame(&self) -> &Vec<Vec<char>> {
        &self.frame
    }


    pub fn clear(&mut self) {
        self.frame = vec![vec!['.'; self.width]; self.height];
    }

    pub fn render_at(&mut self, x: u32, y: u32, object_type: ObjectType) {
        let ch = match object_type {
            Wall => { '#' }
            Player => { '@' }
            Enemy => { 'E' }
            Floor => { '.' }
        };

        self.frame[y as usize][x as usize] = ch;
    }
}
