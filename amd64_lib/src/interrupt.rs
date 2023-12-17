use core::arch::asm;

pub mod apic;
pub mod gdt;
pub mod idt;

/// 割り込みがあるまで、CPUの動きを止める
#[inline(always)]
pub fn halt() {
    unsafe {
        asm!("hlt", options(nomem, nostack, preserves_flags));
    }
}
