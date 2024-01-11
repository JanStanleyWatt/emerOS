#![no_main]
#![no_std]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![feature(const_option)]
#![feature(const_fn_floating_point_arithmetic)]

extern crate alloc;

mod graphic;
mod interrupts;
mod memory;

use amd64_lib::{interrupt::halt, serial_println};
use bootloader_api::{config::Mapping, info::FrameBufferInfo, BootloaderConfig};
use common_lib::locked::Locked;
use core::panic::PanicInfo;
use graphic::text_buffer::TextBuffer;
use once_cell::race::{OnceBox, OnceNonZeroUsize};

// 起動の前準備
// ブート設定
pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.kernel_stack_size = 1000 * 1024;

    // 物理メモリのマッピングを有効化する
    config.mappings.physical_memory = Some(Mapping::Dynamic);

    config
};

// スタティック変数の初期化
static TEXT_BUFFER: OnceBox<Locked<TextBuffer>> = OnceBox::new();
static TEXT_BUFFER_WIDTH: OnceNonZeroUsize = OnceNonZeroUsize::new();
static TEXT_BUFFER_HEIGHT: OnceNonZeroUsize = OnceNonZeroUsize::new();
static FRAME_BUFFER: OnceBox<Locked<&'static mut [u8]>> = OnceBox::new();
static FRAME_BUFFER_INFO: OnceBox<FrameBufferInfo> = OnceBox::new();

// エントリポイントとなる関数へジャンプさせる
bootloader_api::entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

/// エントリポイント
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    interrupts::init();

    memory::init(boot_info.physical_memory_offset, &boot_info.memory_regions);

    let frame_buffer = boot_info.framebuffer.as_mut().unwrap();
    graphic::init(frame_buffer);

    // println!()マクロと画面の描画テスト
    println!("Graphic test");
    println!("グラフィックテスト");
    println!("ｸﾞﾗﾌｨｯｸﾃｽﾄ");
    println!("画面描画試験");
    println!("がめんにもじをかくよ");
    println!("Framebufferテスト now");
    println!("ｸﾞﾗﾌｨｯｸのﾃｽﾄを実施中");

    loop {
        halt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("{}", _info);
    println!("{}", _info);
    loop {
        halt();
    }
}
