use crate::_tests::helpers::rendering_spy::RenderingSpy;
use crate::game::object_type::ObjectType::{Enemy, Player, Wall};
use crate::gfx::renderer::Renderer;

#[test]
fn tiles_are_initially_empty() {
    let renderer = RenderingSpy::new(2, 3);
    assert_eq!(renderer.tiles(), &vec![
        vec!['.', '.'],
        vec!['.', '.'],
        vec!['.', '.']
    ])
}

#[test]
fn renders_objects_as_specific_characters() {
    let mut renderer = RenderingSpy::new(2, 3);
    renderer.render_at(0, 0, Player);
    renderer.render_at(1, 0, Wall);
    renderer.render_at(1, 2, Enemy);

    assert_eq!(renderer.tiles(), &vec![
        vec!['@', '#'],
        vec!['.', '.'],
        vec!['.', 'E']
    ])
}

#[test]
fn tiles_as_string_converts_tiles_to_string() {
    let mut renderer = RenderingSpy::new(2, 3);
    renderer.render_at(0, 0, Player);
    renderer.render_at(1, 0, Wall);
    renderer.render_at(1, 2, Enemy);

    assert_eq!(renderer.tiles_as_string(), "@#\n..\n.E");
}

#[test]
fn clear_replaces_content_with_dots() {
    let mut renderer = RenderingSpy::new(2, 3);
    renderer.render_at(0, 0, Player);
    renderer.render_at(1, 0, Wall);
    renderer.render_at(1, 2, Enemy);
    renderer.clear();

    assert_eq!(renderer.tiles(), &vec![
        vec!['.', '.'],
        vec!['.', '.'],
        vec!['.', '.']
    ])
}

// #[test]
// #[should_panic(expected="assertion failed")]
// fn assert_tiles_fails_if_tiles_do_not_match() {
//     let renderer = RenderingSpy::new(3, 3);
//     renderer.assert_tiles(vec![
//         "@W.",
//         "EE.",
//         "..."
//     ]);
// }
