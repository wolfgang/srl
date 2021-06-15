use std::io::Cursor;
use std::str;

use crate::gfx::TerminalRenderer;

#[test]
fn flush_writes_empty_tiles() {
    let mut renderer = TerminalRenderer::new(3, 2);
    let mut buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    renderer.flush(&mut buffer);
    let written_str = str::from_utf8(buffer.get_ref()).unwrap();
    assert_eq!(written_str, "...\n...");
}