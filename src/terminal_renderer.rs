use std::io::Write;

use crate::object_type::ObjectType::*;
use crate::object_type::ObjectType;
use crate::renderer::Renderer;

pub struct TerminalRenderer {
    frame: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl TerminalRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        let mut renderer = Self {
            width,
            height,
            frame: Vec::new(),
        };
        renderer.clear();
        renderer
    }

    pub fn flush<T: Write>(&mut self, write: &mut T) {
        write.write(self.frame_as_string().as_bytes()).unwrap();
    }

    fn frame_as_string(&self) -> String {
        let strings: Vec<String> = self.frame
            .iter()
            .map(|row| { row.iter().collect() })
            .collect();

        return strings.join("\n");
    }
}

impl Renderer for TerminalRenderer {
    fn clear(&mut self) {
        self.frame = vec![vec!['.'; self.width]; self.height]
    }

    fn render_at(&mut self, x: u32, y: u32, object_type: ObjectType) {
        let ch = match object_type {
            Wall => { '#' }
            Player => { '@' }
            Enemy => { 'E' }
        };

        self.frame[y as usize][x as usize] = ch;
    }
}