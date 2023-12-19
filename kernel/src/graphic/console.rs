use ab_glyph::FontRef;
use alloc::boxed::Box;

use once_cell::race::OnceBox;

use super::{color::Color, text_buffer::TextBuffer};

const FONT_DATA: &[u8; 4259456] = include_bytes!("resources/PlemolJPConsoleNF-Text.ttf");
static TEXT_BUFFER: OnceBox<TextBuffer> = OnceBox::new();

pub(super) fn init() {
    TEXT_BUFFER.get_or_init(|| {
        let font = FontRef::try_from_slice(FONT_DATA).expect("Failed to load font data");
        Box::new(TextBuffer::new(font))
    });

    // テスト
    TEXT_BUFFER
        .get()
        .unwrap()
        .write_char('あ', 20.0, Color::new(255, 255, 255), 0.0, 0.0);
}
