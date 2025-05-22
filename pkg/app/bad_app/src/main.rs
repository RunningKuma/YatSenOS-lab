#![no_std]
#![no_main]

use lib::*;

extern crate lib;

fn main() -> isize {
    let addr = 0xFFFF_FF01_FF11_4514 as *const u8;
    unsafe {
        println!("Trying to access kernel space: {:#x}", *addr);
    }
    0
}

entry!(main);