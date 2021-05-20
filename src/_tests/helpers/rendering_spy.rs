use crate::object_type::ObjectType;
use crate::renderer::Renderer;

pub struct RenderingSpy {}

impl RenderingSpy {
    pub fn new(_width: usize, _height: usize) -> Self {
        Self {}
    }

    pub fn assert_frame(&self, _expected: Vec<&str>) {
        todo!()
    }
}

impl Renderer for RenderingSpy {
    fn render_at(&mut self, _x: u32, _y: u32, _object_type: ObjectType) {}
}