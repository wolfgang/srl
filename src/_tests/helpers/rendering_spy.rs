use crate::object_type::ObjectType;
use crate::renderer::Renderer;

pub struct RenderingSpy {
    frame: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl RenderingSpy {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
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
    fn render_at(&mut self, _x: u32, _y: u32, _object_type: ObjectType) {}
}