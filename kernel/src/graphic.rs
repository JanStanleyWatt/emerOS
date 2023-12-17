use alloc::boxed::Box;
use amd64_lib::memory::allocator::Locked;
use bootloader_api::info::{FrameBuffer, FrameBufferInfo};
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

    // 動作確認（そう遠くないうちに黒に画面を塗りつぶす処理に変更する）
    let info = FRONT_FRAME_BUFFER.get().unwrap().info;
    let height = info.height;
    let width = info.stride;
    let pixel = info.bytes_per_pixel;
    let color = [255u8, 255, 255];
    {
        let mut buf = FRONT_FRAME_BUFFER.get().unwrap().frame_buffer.lock();
        for y in 0..height {
            for x in 0..width {
                let index = (y * width + x) * pixel;
                buf[index..index + 3].copy_from_slice(&color);
            }
        }
    }
}

struct FrontFrameBuffer {
    pub(crate) frame_buffer: Locked<&'static mut [u8]>,
    pub(crate) info: FrameBufferInfo,
}
