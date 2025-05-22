#![no_std]
#![no_main]

use lib::*;

extern crate lib;

fn main() -> isize {
    println!("Hello, world!!!");
    // let k = sys_get_pid();
    // println!("My pid is: {}", k);
    sys_exit(233);
    
}

entry!(main);
