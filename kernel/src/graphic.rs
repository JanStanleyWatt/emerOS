//! グラフィック関連の機能を定義するモジュール

pub(crate) mod console;
pub(crate) mod text_buffer;

use ab_glyph::FontRef;
use alloc::boxed::Box;
use bootloader_api::info::FrameBuffer;
use common_lib::locked::Locked;

use crate::{graphic::text_buffer::TextBuffer, FRAME_BUFFER, FRAME_BUFFER_INFO, TEXT_BUFFER};

const FONT_TEXT: &[u8; 4259456] = include_bytes!("graphic/resources/PlemolJPConsoleNF-Text.ttf");
const FONT_BOLD: &[u8; 4257220] = include_bytes!("graphic/resources/PlemolJPConsoleNF-Bold.ttf");
const FONT_SCALE: f32 = 24.0;

/// 描画モジュールの初期化
pub(crate) fn init(frame_buffer: &'static mut FrameBuffer) {
    FRAME_BUFFER_INFO.get_or_init(|| Box::new(frame_buffer.info()));
    FRAME_BUFFER.get_or_init(|| Box::new(Locked::new(frame_buffer.buffer_mut())));

    // Lock
    {
        FRAME_BUFFER.get().unwrap().lock().fill(0);
    } // Unlock

    TEXT_BUFFER.get_or_init(|| {
        let font_text = FontRef::try_from_slice(FONT_TEXT).expect("Failed to load text font data");
        let font_bold = FontRef::try_from_slice(FONT_BOLD).expect("Failed to load bold font data");
        Box::new(Locked::new(TextBuffer::new(
            font_text,
            font_bold,
            FONT_SCALE,
            FRAME_BUFFER.get().unwrap(),
            FRAME_BUFFER_INFO.get().unwrap(),
        )))
    });
}
