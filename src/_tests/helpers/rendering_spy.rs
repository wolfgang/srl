use crate::game::object_type::ObjectType;
use crate::gfx::renderer::Renderer;
use crate::gfx::string_backend::StringBackend;

pub struct RenderingSpy {
    backend: StringBackend
}

impl RenderingSpy {
    pub fn new(width: usize, height: usize) -> Self {
        Self { backend: StringBackend::new(width, height) }
    }

    pub fn tiles_as_string(&self) -> String {
        self.backend.tiles_as_string()
    }

    pub fn tiles(&self) -> &Vec<Vec<char>> {
        self.backend.tiles()
    }

    pub fn assert_tiles(&self, expected: Vec<&str>) {
        assert_eq!(self.tiles_as_string(), expected.join("\n").replace(" ", ""));
    }

    pub fn combat_log(&self) -> Vec<&str> {
        self.backend.combat_log()
    }

    pub fn assert_combat_log(&self, _expected: Vec<&str>) {
        todo!()
    }
}

impl Renderer for RenderingSpy {
    fn clear(&mut self) {
        self.backend.clear();
    }

    fn render_at(&mut self, x: u32, y: u32, object_type: ObjectType) {
        self.backend.render_at(x, y, object_type);
    }

    fn append_combat_log(&mut self, text: &str) {
        self.backend.append_combat_log(text)
    }
}