use std::io::Write;

use console::style;

use crate::game::object_type::ObjectType;
use crate::gfx::renderer::Renderer;
use crate::gfx::string_backend::StringBackend;

pub struct TerminalRenderer {
    backend: StringBackend,
}

impl TerminalRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self { backend: StringBackend::new(width, height) }
    }

    pub fn flush<T: Write>(&mut self, write: &mut T) {
        write.write(format!("{}", self.tiles_as_string()).as_bytes()).unwrap();
        write.flush().unwrap();
    }

    fn tiles_as_string(&self) -> String {
        let mut output = self.backend.tiles_as_string();
        match output.find('@') {
            Some(offset) => {
                output.replace_range(offset..offset + 1, style("@").red().to_string().as_str());
            }
            _ => {}
        }

        return output;
    }
}

impl Renderer for TerminalRenderer {
    fn clear(&mut self) {
        self.backend.clear();
    }

    fn render_at(&mut self, x: u32, y: u32, object_type: ObjectType) {
        self.backend.render_at(x, y, object_type);
    }
}