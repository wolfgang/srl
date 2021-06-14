use crate::game::object_type::ObjectType::*;
use crate::game::object_type::ObjectType;
use crate::gfx::renderer::Renderer;

pub struct RenderingSpy {
    frame: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl RenderingSpy {
    pub fn new(width: usize, height: usize) -> Self {
        let mut renderer = Self {
            width,
            height,
            frame: Vec::new(),
        };
        renderer.clear();
        renderer
    }

    pub fn frame_as_string(&self) -> String {
        let strings: Vec<String> = self.frame
            .iter()
            .map(|row| { row.iter().collect() })
            .collect();

        return strings.join("\n");
    }


    pub fn frame(&self) -> &Vec<Vec<char>> {
        &self.frame
    }

    pub fn assert_frame(&self, expected: Vec<&str>) {
        assert_eq!(self.frame_as_string(), expected.join("\n").replace(" ", ""));
    }

    pub fn assert_combat_log(&self, _expected: Vec<&str>) {
        todo!()
    }
}

impl Renderer for RenderingSpy {
    fn clear(&mut self) {
        self.frame = vec![vec!['.'; self.width]; self.height]
    }

    fn render_at(&mut self, x: u32, y: u32, object_type: ObjectType) {
        let ch = match object_type {
            Wall => { '#' }
            Player => { '@' }
            Enemy => { 'E' }
            Floor => { '.'}
        };

        self.frame[y as usize][x as usize] = ch;
    }
}