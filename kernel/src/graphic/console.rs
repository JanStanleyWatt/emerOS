use ab_glyph::FontRef;
use alloc::boxed::Box;

use once_cell::race::OnceBox;

use super::{
    color::Color,
    text_buffer::{FontType, TextBuffer},
};

const FONT_TEXT: &[u8; 4259456] = include_bytes!("resources/PlemolJPConsoleNF-Text.ttf");
const FONT_BOLD: &[u8; 4257220] = include_bytes!("resources/PlemolJPConsoleNF-Bold.ttf");
static TEXT_BUFFER: OnceBox<TextBuffer> = OnceBox::new();

pub(super) fn init() {
    TEXT_BUFFER.get_or_init(|| {
        let font_text = FontRef::try_from_slice(FONT_TEXT).expect("Failed to load text font data");
        let font_bold = FontRef::try_from_slice(FONT_BOLD).expect("Failed to load bold font data");
        Box::new(TextBuffer::new(font_text, font_bold, 20.0))
    });

    // テスト
    TEXT_BUFFER.get().unwrap().write_char(
        'あ',
        FontType::Text,
        Color::new(255, 255, 255),
        0.0,
        0.0,
    );
    TEXT_BUFFER.get().unwrap().write_char(
        'あ',
        FontType::Bold,
        Color::new(255, 255, 255),
        20.0,
        0.0,
    );
}
