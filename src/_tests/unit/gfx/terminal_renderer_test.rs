use std::io::Cursor;
use std::str;

use crate::game::object_type::ObjectType::*;
use crate::gfx::{Renderer, TerminalRenderer};

#[test]
fn flush_writes_empty_tiles() {
    let mut renderer = TerminalRenderer::new(3, 2);
    let mut buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    renderer.flush(&mut buffer);
    let written_str = str::from_utf8(buffer.get_ref()).unwrap();
    assert_eq!(written_str, "...\n...");
}

#[test]
fn flush_writes_previously_rendered_tiles() {
    let mut renderer = TerminalRenderer::new(2, 2);
    let mut buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    renderer.render_at(0, 0, Player);
    renderer.render_at(1, 0, Wall);
    renderer.render_at(0, 1, Enemy);
    renderer.render_at(1, 1, Floor);


    renderer.flush(&mut buffer);
    let written_str = str::from_utf8(buffer.get_ref()).unwrap();
    assert_eq!(written_str, "@#\nE.");
}

#[test]
fn clear_resets_tiles_to_empty() {
    let mut renderer = TerminalRenderer::new(2, 2);
    let mut buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    renderer.render_at(0, 0, Player);
    renderer.render_at(1, 0, Wall);
    renderer.render_at(0, 1, Enemy);
    renderer.render_at(1, 1, Floor);
    renderer.clear();
    renderer.flush(&mut buffer);
    let written_str = str::from_utf8(buffer.get_ref()).unwrap();
    assert_eq!(written_str, "..\n..");


}