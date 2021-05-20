use crate::_tests::helpers::rendering_spy::RenderingSpy;
use crate::object_type::ObjectType::{Enemy, Player, Wall};
use crate::renderer::Renderer;

#[test]
fn frame_is_initially_empty() {
    let renderer = RenderingSpy::new(2, 3);
    assert_eq!(renderer.frame(), &vec![
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

    assert_eq!(renderer.frame(), &vec![
        vec!['@', 'W'],
        vec!['.', '.'],
        vec!['.', 'E']
    ])
}