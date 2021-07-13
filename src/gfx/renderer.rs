use crate::game::object_type::ObjectType;

pub trait Renderer {
    fn clear(&mut self);
    fn render_tile(&mut self, x: u32, y: u32, object_type: ObjectType);
    fn append_combat_log(&mut self, text: &str);
    fn render_player_hp(&mut self, _value: u32);
}