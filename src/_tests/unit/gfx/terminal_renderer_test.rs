use std::io::Cursor;
use std::str;

use crate::game::object_type::ObjectType::*;
use crate::gfx::{Renderer, TerminalRenderer};

#[test]
fn flush_writes_empty_tiles() {
    let mut renderer = TerminalRenderer::new(3, 2);
    verify_flush_writes("...\n...", &mut renderer);
}

#[test]
fn flush_writes_previously_rendered_tiles() {
    let mut renderer = TerminalRenderer::new(2, 2);

    renderer.render_at(0, 0, Player);
    renderer.render_at(1, 0, Wall);
    renderer.render_at(0, 1, Enemy);
    renderer.render_at(1, 1, Floor);

    verify_flush_writes("@#\nE.", &mut renderer);
}

#[test]
fn clear_resets_tiles_to_empty() {
    let mut renderer = TerminalRenderer::new(2, 2);

    renderer.render_at(0, 0, Player);
    renderer.render_at(1, 0, Wall);
    renderer.render_at(0, 1, Enemy);
    renderer.render_at(1, 1, Floor);
    renderer.clear();

    verify_flush_writes("..\n..", &mut renderer);
}

fn verify_flush_writes(expected: &str, renderer: &mut TerminalRenderer) {
    let mut buffer = Cursor::new(Vec::new());
    renderer.flush(&mut buffer);
    let actual = str::from_utf8(&buffer.get_ref()).unwrap();
    assert_eq!(actual, expected);
}

