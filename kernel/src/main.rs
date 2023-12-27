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
use core::{
    fmt::{self, Write},
    panic::PanicInfo,
};
use graphic::text_buffer::TextBuffer;
use once_cell::race::OnceBox;

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
    // println!("OK");

    loop {
        halt();
    }
}

// 以下、基本的な画面出力をサポートするマクロ等

/// 画面に文字列を描画するマクロ。graphic::init()の処理が終わってから使用すること
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

/// 画面に文字列を描画し、最後に改行を行うマクロ。graphic::init()の処理が終わってから使用すること
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    TEXT_BUFFER.get().unwrap().lock().write_fmt(args).unwrap();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("{}", _info);
    println!("{}", _info);
    loop {
        halt();
    }
}
