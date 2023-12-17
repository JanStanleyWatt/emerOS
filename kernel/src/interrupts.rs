use amd64_lib::interrupt::{gdt, idt};

pub(crate) fn init() {
    // 割り込み初期化
    gdt::init();
    idt::init();
}
