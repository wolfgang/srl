use crate::object_type::ObjectType;

pub trait Renderer {
    fn render_at(&mut self, x: u32, y: u32, object_type: ObjectType);
}