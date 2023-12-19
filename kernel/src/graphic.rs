pub(crate) mod color;
pub(crate) mod console;
pub(crate) mod text_buffer;

use alloc::boxed::Box;
use bootloader_api::info::{FrameBuffer, FrameBufferInfo};
use common_lib::locked::Locked;
use once_cell::race::OnceBox;

static FRONT_FRAME_BUFFER: OnceBox<FrontFrameBuffer> = OnceBox::new();

/// 描画モジュールの初期化
pub(crate) fn init(frame_buffer: &'static mut FrameBuffer) {
    FRONT_FRAME_BUFFER.get_or_init(|| {
        // `frame_buffer`は&mutで渡してしまうので、先にFrameBufferInfoを取り出す
        let info = frame_buffer.info();

        Box::new(FrontFrameBuffer {
            frame_buffer: Locked::new(frame_buffer.buffer_mut()),
            info,
        })
    });

    // Lock
    {
        FRONT_FRAME_BUFFER
            .get()
            .unwrap()
            .frame_buffer
            .lock()
            .fill(0);
    } // Unlock

    console::init();
}

struct FrontFrameBuffer {
    pub(crate) frame_buffer: Locked<&'static mut [u8]>,
    pub(crate) info: FrameBufferInfo,
}
