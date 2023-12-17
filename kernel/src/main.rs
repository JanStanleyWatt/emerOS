#![no_main]
#![no_std]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]

extern crate alloc;

mod graphic;
mod interrupts;
mod memory;

use core::panic::PanicInfo;

use amd64_lib::{interrupt::halt, serial_println};
use bootloader_api::{config::Mapping, BootloaderConfig};

// 起動の前準備
// ブート設定
pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.kernel_stack_size = 1000 * 1024;

    // 物理メモリのマッピングを有効化する
    config.mappings.physical_memory = Some(Mapping::Dynamic);

    config
};

// エントリポイントとなる関数へジャンプさせる
bootloader_api::entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

// エントリポイント
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    interrupts::init();

    memory::init(boot_info.physical_memory_offset, &boot_info.memory_regions);

    let frame_buffer = boot_info.framebuffer.as_mut().unwrap();
    graphic::init(frame_buffer);

    serial_println!("OK");

    loop {
        halt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("{}", _info);
    loop {
        halt();
    }
}
