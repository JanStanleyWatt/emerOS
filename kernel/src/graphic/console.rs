//! コンソール機能を定義するモジュール

use ab_glyph::FontRef;
use alloc::boxed::Box;

use common_lib::locked::Locked;

use crate::TEXT_BUFFER;

use super::{text_buffer::TextBuffer, FRAME_BUFFER, FRAME_BUFFER_INFO};

const FONT_TEXT: &[u8; 4259456] = include_bytes!("resources/PlemolJPConsoleNF-Text.ttf");
const FONT_BOLD: &[u8; 4257220] = include_bytes!("resources/PlemolJPConsoleNF-Bold.ttf");

pub(super) fn init() {
    TEXT_BUFFER.get_or_init(|| {
        let font_text = FontRef::try_from_slice(FONT_TEXT).expect("Failed to load text font data");
        let font_bold = FontRef::try_from_slice(FONT_BOLD).expect("Failed to load bold font data");
        Box::new(Locked::new(TextBuffer::new(
            font_text,
            font_bold,
            24.0,
            FRAME_BUFFER.get().unwrap(),
            FRAME_BUFFER_INFO.get().unwrap(),
        )))
    });
}
