use super::LocalApic;
use bit_field::BitField;
use core::fmt::{Debug, Error, Formatter};
use core::ptr::{read_volatile, write_volatile};
use x86::cpuid::CpuId;
use crate::interrupt::consts::Interrupts;
use crate::interrupt::consts::Irq;
use crate::memory::address::physical_to_virtual;
use bitflags::{bitflags, Bits};
/// Default physical address of xAPIC
pub const LAPIC_ADDR: u64 = 0xFEE00000;

pub struct XApic {
    addr: u64,
}

impl XApic {
    pub unsafe fn new(addr: u64) -> Self {
        XApic { addr }
    }

    unsafe fn read(&self, reg: u32) -> u32 {
        unsafe {
            read_volatile((self.addr + reg as u64) as *const u32)
        }
    }

    unsafe fn write(&mut self, reg: u32, value: u32) {
        unsafe {
            write_volatile((self.addr + reg as u64) as *mut u32, value);
            self.read(0x20);
        }
    }
}



bitflags! {
        pub struct Flag:  u32 {
            const ENABLED = 1 << 8;
            const DISABLED = 1 << 16;
            const MASK = 0xFF;
            const IVT_TIMER = 0x320;
            const SPIV = 0xF0;
            const DIVIDE_CONFIG = 0x3E0;
            const ICR_T = 0x380;
            const LINT0 = 0x350;
            const LINT1 = 0x360;
            const PCINT = 0x340;
            const LVTER = 0x370;
            const ESR = 0x280;
            const EOI = 0x0B0;
            const ICR1 = 0x310;
            const ICR0 = 0x300;
            const TPR = 0x080;
        }
    }

impl LocalApic for XApic {
    /// If this type APIC is supported
    fn support() -> bool {
        // FIXME: Check CPUID to see if xAPIC is supported.
        CpuId::new().get_feature_info().map(|f| f.has_apic()).unwrap_or(false)
    }

    /// Initialize the xAPIC for the current CPU.
    fn cpu_init(&mut self) {
        unsafe {
            // FIXME: Enable local APIC; set spurious interrupt vector.
            let mut spiv = self.read(Flag::SPIV.bits());
            spiv |= Flag::ENABLED.bits();
            spiv &= !(Flag::MASK.bits());
            spiv |= Interrupts::IrqBase as u32 + Irq::Spurious as u32;
            self.write(Flag::SPIV.bits(), spiv);
            // FIXME: The timer repeatedly counts down at bus frequency
            self.write(Flag::DIVIDE_CONFIG.bits(),0b1011);
            self.write(Flag::ICR_T.bits(), 0x20000);
            
            let mut lvt_timer = self.read(Flag::IVT_TIMER.bits());// clear and set Vector
            lvt_timer &= !(Flag::MASK.bits());
            lvt_timer |= Interrupts::IrqBase as u32 + Irq::Timer as u32;
            lvt_timer &= !(Flag::DISABLED.bits()); // clear Mask
            lvt_timer |= 1 << 17; // set Timer Periodic Mode
            self.write(Flag::IVT_TIMER.bits(), lvt_timer);

            // FIXME: Disable logical interrupt lines (LINT0, LINT1)
            self.write(Flag::LINT0.bits(), Flag::DISABLED.bits());
            self.write(Flag::LINT1.bits(), Flag::DISABLED.bits());
           
            // FIXME: Disable performance counter overflow interrupts (PCINT)
            self.write(Flag::PCINT.bits(), Flag::DISABLED.bits());
            // FIXME: Map error interrupt to IRQ_ERROR.
            let mut lvt_error = self.read(Flag::LVTER.bits());
            lvt_error &= !(Flag::MASK.bits());
            lvt_error |= Interrupts::IrqBase as u32 + Irq::Error as u32;
            self.write(Flag::LVTER.bits(), lvt_error);
            // FIXME: Clear error status register (requires back-to-back writes).
            self.write(Flag::ESR.bits(), 0);
            self.write(Flag::ESR.bits(), 0);
            // FIXME: Ack any outstanding interrupts.
            self.eoi();
            // FIXME: Send an Init Level De-Assert to synchronise arbitration ID's.
            self.write(Flag::ICR1.bits(), 0); // set ICR 0x310
            const BCAST: u32 = 1 << 19;
            const INIT: u32 = 5 << 8;
            const TMLV: u32 = 1 << 15; // TM = 1, LV = 0
            self.write(Flag::ICR0.bits(), BCAST | INIT | TMLV); // set ICR 0x300
            const DS: u32 = 1 << 12;
            while self.read(Flag::ICR0.bits()) & DS != 0 {} // wait for delivery status
            // FIXME: Enable interrupts on the APIC (but not on the processor).
            self.write(Flag::TPR.bits(), 0);
            
        }

        // NOTE: Try to use bitflags! macro to set the flags.
    }

    fn id(&self) -> u32 {
        // NOTE: Maybe you can handle regs like `0x0300` as a const.
        unsafe { self.read(0x0020) >> 24 }
    }


    fn version(&self) -> u32 {
        unsafe { self.read(0x0030) }
    }

    fn icr(&self) -> u64 {
        unsafe { (self.read(0x0310) as u64) << 32 | self.read(0x0300) as u64 }
    }

    fn set_icr(&mut self, value: u64) {
        unsafe {
            while self.read(0x0300).get_bit(12) {}
            self.write(0x0310, (value >> 32) as u32);
            self.write(0x0300, value as u32);
            while self.read(0x0300).get_bit(12) {}
        }
    }

    fn eoi(&mut self) {
        unsafe {
            self.write(0x00B0, 0);
        }
    }
}

impl Debug for XApic {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Xapic")
            .field("id", &self.id())
            .field("version", &self.version())
            .field("icr", &self.icr())
            .finish()
    }
}
