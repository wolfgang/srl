use crate::game::object_type::ObjectType;

pub trait Renderer {
    fn clear(&mut self);
    fn render_at(&mut self, x: u32, y: u32, object_type: ObjectType);
}