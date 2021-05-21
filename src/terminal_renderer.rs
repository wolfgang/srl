use std::io::Cursor;

use crate::object_type::ObjectType;
use crate::renderer::Renderer;

pub struct TerminalRenderer {
    render_buffer: Cursor<Vec<u8>>,
}

impl TerminalRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self { render_buffer: Cursor::new(Vec::with_capacity(width * height)) }
    }
}

impl Renderer for TerminalRenderer {
    fn clear(&mut self) {}

    fn render_at(&mut self, x: u32, y: u32, object_type: ObjectType) {}
}