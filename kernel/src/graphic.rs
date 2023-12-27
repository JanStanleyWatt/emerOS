//! グラフィック関連の機能を定義するモジュール

pub(crate) mod color;
pub(crate) mod console;
pub(crate) mod text_buffer;

use alloc::boxed::Box;
use bootloader_api::info::FrameBuffer;
use common_lib::locked::Locked;

use crate::{FRAME_BUFFER, FRAME_BUFFER_INFO};

/// 描画モジュールの初期化
pub(crate) fn init(frame_buffer: &'static mut FrameBuffer) {
    FRAME_BUFFER_INFO.get_or_init(|| Box::new(frame_buffer.info()));
    FRAME_BUFFER.get_or_init(|| Box::new(Locked::new(frame_buffer.buffer_mut())));

    // Lock
    {
        FRAME_BUFFER.get().unwrap().lock().fill(0);
    } // Unlock

    console::init();
}
