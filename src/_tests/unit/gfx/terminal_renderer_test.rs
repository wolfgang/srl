use std::io::Cursor;
use std::str;

use crate::game::object_type::ObjectType::*;
use crate::gfx::{Renderer, TerminalRenderer};

#[test]
fn flush_writes_empty_tiles() {
    let mut renderer = TerminalRenderer::new(3, 2);
    verify_flush_writes(&mut renderer, vec![
        "...",
        "..."]);
}

#[test]
fn flush_writes_previously_rendered_tiles() {
    let mut renderer = TerminalRenderer::new(2, 2);
    renderer.disable_colors();

    renderer.render_tile(0, 0, Player);
    renderer.render_tile(1, 0, Wall);
    renderer.render_tile(0, 1, Enemy);

    verify_flush_writes(&mut renderer, vec!["@#", "E."]);
}

#[test]
fn clear_resets_tiles_to_empty() {
    let mut renderer = TerminalRenderer::new(2, 2);

    renderer.render_tile(0, 0, Player);
    renderer.render_tile(1, 0, Wall);
    renderer.render_tile(0, 1, Enemy);
    renderer.clear();

    verify_flush_writes(&mut renderer, vec!["..", ".."]);
}

#[test]
fn renders_combat_log_to_the_right_of_dungeon() {
    let mut renderer = TerminalRenderer::new(3, 2);
    renderer.append_combat_log("combat log line 1");
    renderer.append_combat_log("combat log line 2");

    verify_flush_writes(&mut renderer, vec![
        "...  combat log line 1",
        "...  combat log line 2"]);
}

#[test]
fn clear_clears_combat_log() {
    let mut renderer = TerminalRenderer::new(3, 2);
    renderer.append_combat_log("combat log line 1");
    renderer.append_combat_log("combat log line 2");
    renderer.clear();

    verify_flush_writes(&mut renderer, vec![
        "...",
        "..."]);
}

#[test]
fn render_player_hp_displays_player_hp_on_top() {
    let mut renderer = TerminalRenderer::new(10, 2);
    renderer.render_player_hp(123);
    verify_flush_writes(&mut renderer, vec![
        "HP: 123",
        "..........",
        ".........."]);
}

#[test]
fn flush_returns_number_of_lines_written() {
    let mut renderer = TerminalRenderer::new(2, 3);
    renderer.render_player_hp(123);
    assert_eq!(renderer.flush(&mut cursor_buffer()), 4);


}

fn verify_flush_writes(renderer: &mut TerminalRenderer, expected: Vec<&str>) {
    let mut buffer = cursor_buffer();
    renderer.flush(&mut buffer);
    let actual = str::from_utf8(&buffer.get_ref()).unwrap();
    assert_eq!(actual, expected.join("\n"));
}

fn cursor_buffer() -> Cursor<Vec<u8>> {
    Cursor::new(Vec::new())
}

