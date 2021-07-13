use std::io::Write;

use console::style;

use crate::game::object_type::ObjectType;
use crate::gfx::renderer::Renderer;
use crate::gfx::string_backend::StringBackend;

pub struct TerminalRenderer {
    backend: StringBackend,
    colors_enabled: bool
}

impl TerminalRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self { backend: StringBackend::new(width, height), colors_enabled: true }
    }

    pub fn flush<T: Write>(&mut self, write: &mut T) {
        write.write(format!("{}", self.frame_as_string()).as_bytes()).unwrap();
        write.flush().unwrap();
    }

    pub fn disable_colors(&mut self) {
        self.colors_enabled = false;
    }

    fn frame_as_string(&self) -> String {
        let combat_log = self.backend.combat_log();
        let tile_lines = self.backend.tiles_as_strings();
        let mut result = Vec::with_capacity(tile_lines.len());
        for (index, tiles_line) in tile_lines.iter().enumerate() {
            let mut tiles_line = tiles_line.clone();
            if self.colors_enabled { Self::color_player(&mut tiles_line) }
            result.push(format!("{}{}", tiles_line, Self::combat_log_line_at(index, &combat_log)));
        }

        return result.join("\n");
    }

    fn combat_log_line_at(index: usize, combat_log: &Vec<&str>) -> String {
        if index < combat_log.len() {
            format!("  {}", combat_log[index])
        } else {
            "".into()
        }
    }

    fn color_player(tiles_line: &mut String) {
        if let Some(offset) = tiles_line.find('@') {
            tiles_line.replace_range(offset..offset + 1, style("@").red().to_string().as_str());
        }
    }
}

impl Renderer for TerminalRenderer {
    fn clear(&mut self) {
        self.backend.clear();
    }

    fn render_tile(&mut self, x: u32, y: u32, object_type: ObjectType) {
        self.backend.render_tile(x, y, object_type);
    }

    fn append_combat_log(&mut self, text: &str) {
        self.backend.append_combat_log(text)
    }

    fn render_player_hp(&mut self, _value: u32) {
        todo!()
    }
}