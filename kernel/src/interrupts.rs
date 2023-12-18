/// 割り込みなどの初期化
#[cfg(target_arch = "x86_64")]
pub(crate) fn init() {
    use amd64_lib::interrupt::{gdt, idt};

    gdt::init();
    idt::init();
}
