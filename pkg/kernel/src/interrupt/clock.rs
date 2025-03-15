use super::consts::*;
use core::sync::atomic::AtomicU64;
use x86_64::structures::idt::{InterruptDescriptorTable,InterruptStackFrame};

pub unsafe fn register_idt(idt: &mut InterruptDescriptorTable) {
    idt[Interrupts::IrqBase as u8 + Irq::Timer as u8]
        .set_handler_fn(clock_handler);
}

pub extern "x86-interrupt" fn clock_handler(_sf: InterruptStackFrame) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        if inc_counter() % 0x20000 == 0 {
            info!("Tick! @{}", read_counter());
        }
        super::ack();
    });
}

static COUNTER: AtomicU64 = AtomicU64::new(0);
/// Read the current counter value.

#[inline]
pub fn read_counter() -> u64 {
    // FIXME: load counter value
    COUNTER.load(core::sync::atomic::Ordering::Relaxed) as u64
}

#[inline]
pub fn inc_counter() -> u64 {
    // FIXME: read counter value and increase it
    COUNTER.fetch_add(1, core::sync::atomic::Ordering::Relaxed) as u64
}






