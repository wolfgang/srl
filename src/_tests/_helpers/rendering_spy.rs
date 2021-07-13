use crate::game::object_type::ObjectType;
use crate::gfx::renderer::Renderer;
use crate::gfx::string_backend::StringBackend;

pub struct RenderingSpy {
    backend: StringBackend,
    current_player_hp: u32
}

impl RenderingSpy {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            backend: StringBackend::new(width, height),
            current_player_hp: 0
        }
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

    pub fn assert_combat_log(&self, expected: Vec<&str>) {
        let expected_str = expected.join("\n");
        let actual_str = self.combat_log().join("\n");
        assert_eq!(actual_str, expected_str);
    }

    pub fn assert_player_hp_rendered(&self, expected: u32) {
        assert_eq!(self.current_player_hp, expected)
    }


    pub fn combat_log(&self) -> Vec<&str> {
        self.backend.combat_log()
    }
}

impl Renderer for RenderingSpy {
    fn clear(&mut self) {
        self.backend.clear();
    }

    fn render_tile(&mut self, x: u32, y: u32, object_type: ObjectType) {
        self.backend.render_tile(x, y, object_type);
    }

    fn append_combat_log(&mut self, text: &str) {
        self.backend.append_combat_log(text)
    }

    fn render_player_hp(&mut self, value: u32) {
        self.current_player_hp = value;
    }
}