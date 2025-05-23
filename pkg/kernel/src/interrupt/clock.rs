use super::consts::*;
use core::sync::atomic::AtomicU64;
use x86_64::structures::idt::{InterruptDescriptorTable,InterruptStackFrame};
use crate::{memory::gdt, proc::ProcessContext};

pub unsafe fn register_idt(idt: &mut InterruptDescriptorTable) {
    unsafe{
    idt[Interrupts::IrqBase as u8 + Irq::Timer as u8]
        .set_handler_fn(process_switcher_handler)
        .set_stack_index(gdt::CLOCK_IST_INDEX);
    }
}

pub extern "C" fn process_switcher(mut context: ProcessContext) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        if inc_counter() % 0x100 == 0 { // 设置时间片为 100 
            crate::proc::switch(&mut context);
        }
        super::ack();
    });
}

as_handler!(process_switcher);



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






