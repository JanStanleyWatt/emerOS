//! グラフィック関連の機能を定義するモジュール
//! 出力できる文字は [PlemolJP Console Nerds](https://github.com/yuru7/PlemolJP) に収録されたもののうち、
//! TextとBoldの二種類の太さのうちどれか一つに限る
//!
//!
//! ## LICENSE
//! Copyright (c) 2021, Yuko OTAWARA. with Reserved Font Name "PlemolJP"
//!
//! This Font Software is licensed under the SIL Open Font License, Version 1.1.
//! This license is copied below, and is also available with a FAQ at:
//! https://scripts.sil.org/OFL

pub(crate) mod color;
pub(crate) mod console;
pub(crate) mod text_buffer;

use core::num::NonZeroUsize;

use ab_glyph::FontRef;
use alloc::boxed::Box;
use bootloader_api::info::FrameBuffer;
use common_lib::locked::Locked;

use self::text_buffer::TextBuffer;
use crate::{FRAME_BUFFER, FRAME_BUFFER_INFO, TEXT_BUFFER, TEXT_BUFFER_HEIGHT, TEXT_BUFFER_WIDTH};

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
            font_text, font_bold, FONT_SCALE,
        )))
    });

    let info = FRAME_BUFFER_INFO.get().unwrap();

    let width = info.stride / (FONT_SCALE / 2.0) as usize;
    let height = info.height / FONT_SCALE as usize;

    // WIDTHもHEIGHTも割り算で求めるため、`info.stride`や`info.stride`が0でない限り（まずありえない）0を渡すことは無い
    TEXT_BUFFER_WIDTH.get_or_init(move || unsafe { NonZeroUsize::new_unchecked(width) });
    TEXT_BUFFER_HEIGHT.get_or_init(move || unsafe { NonZeroUsize::new_unchecked(height) });
}
