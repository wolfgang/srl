use crate::_tests::helpers::rendering_spy::RenderingSpy;

#[test]
fn frame_is_initially_empty() {
    let renderer = RenderingSpy::new(2, 3);
    assert_eq!(renderer.frame(), &vec![
        vec!['.', '.'],
        vec!['.', '.'],
        vec!['.', '.']
    ])
}