use std::io::Write;

use console::style;

use crate::game::object_type::ObjectType::*;
use crate::game::object_type::ObjectType;
use crate::gfx::renderer::Renderer;

// use console::style;

pub struct TerminalRenderer {
    frame: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl TerminalRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        let mut renderer = Self {
            width,
            height,
            frame: Vec::new(),
        };
        renderer.clear();
        renderer
    }

    pub fn flush<T: Write>(&mut self, write: &mut T) {
        let frame_as_string = self.frame_as_string();
        write.write(format!("{}", frame_as_string).as_bytes()).unwrap();
        write.flush().unwrap();
    }

    fn frame_as_string(&self) -> String {
        let strings: Vec<String> = self.frame
            .iter()
            .map(|row| { row.iter().collect() })
            .collect();


        let mut output = strings.join("\n");
        match output.find('@') {
            Some(offset) => {
                output.replace_range(offset..offset+1, style("@").red().to_string().as_str());
            }
            _ => {}
        }

        return output;
    }
}

impl Renderer for TerminalRenderer {
    fn clear(&mut self) {
        self.frame = vec![vec!['.'; self.width]; self.height];
    }

    fn render_at(&mut self, x: u32, y: u32, object_type: ObjectType) {
        let ch = match object_type {
            Wall => { '#' }
            Player => { '@' }
            Enemy => { 'E' }
            Floor => { '.'}
        };

        self.frame[y as usize][x as usize] = ch;
    }
}