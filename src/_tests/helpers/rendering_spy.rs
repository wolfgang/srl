use crate::object_type::ObjectType::*;
use crate::object_type::ObjectType;
use crate::renderer::Renderer;

pub struct RenderingSpy {
    frame: Vec<Vec<char>>,
}

impl RenderingSpy {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            frame: vec![vec!['.'; width]; height],
        }
    }

    pub fn frame(&self) -> &Vec<Vec<char>> {
        &self.frame
    }

    pub fn assert_frame(&self, _expected: Vec<&str>) {
        todo!()
    }
}

impl Renderer for RenderingSpy {
    fn render_at(&mut self, x: u32, y: u32, object_type: ObjectType) {
        let ch = match object_type {
            Wall => { 'W' }
            Player => { '@' }
            Enemy => { 'E' }
        };

        self.frame[y as usize][x as usize] = ch;
    }
}