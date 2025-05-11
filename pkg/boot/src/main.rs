#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate log;
extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use elf::load_elf;
use elf::map_physical_memory;
use elf::map_range;
use uefi::boot::exit_boot_services;
use uefi::{Status, entry, mem::memory_map::MemoryMap};
use x86_64::registers::control::*;
use xmas_elf::ElfFile;
use ysos_boot::config as ysos_config;
use ysos_boot::*;

mod config;

const CONFIG_PATH: &str = "\\EFI\\BOOT\\boot.conf";

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init().expect("Failed to initialize utilities");

    log::set_max_level(log::LevelFilter::Debug);
    info!("Running UEFI bootloader...");

    // 1. Load config
    let config = {
        /* FIXME: Load config file as Config */
        let mut file = open_file(CONFIG_PATH);
        let buf = load_file(&mut file);//following the doc to load file
        crate::config::Config::parse(buf)//load into the Config struct
    };

    info!("Config: {:#x?}", config);

    // 2. Load ELF files
    let elf = {
        /* FIXME: Load kernel elf file */
        let mut file = open_file(config.kernel_path);
        let buf = load_file(&mut file);//by doc
        ElfFile::new(buf).unwrap()//load into the ElfFile struct and unwarp
    };

    unsafe {
        set_entry(elf.header.pt2.entry_point() as usize);
    }

    // 3. Load MemoryMap
    let mmap = uefi::boot::memory_map(MemoryType::LOADER_DATA).expect("Failed to get memory map");

    let max_phys_addr = mmap
        .entries()
        .map(|m| m.phys_start + m.page_count * 0x1000)
        .max()
        .unwrap()
        .max(0x1_0000_0000); // include IOAPIC MMIO area

    // 4. Map ELF segments, kernel stack and physical memory to virtual memory
    let mut page_table = current_page_table();

    // FIXME: root page table is readonly, disable write protect (Cr0)
    unsafe {
        Cr0::update(|cr0| cr0.remove(Cr0Flags::WRITE_PROTECT));//disable write protect!
    }

    // FIXME: map physical memory to specific virtual address offset
    map_physical_memory(
        config.physical_memory_offset,
        max_phys_addr,
        &mut page_table,
        &mut UEFIFrameAllocator,//can pass as parameter
    );
    // FIXME: load and map the kernel elf file
    load_elf(
        &elf,
        config.physical_memory_offset,
        &mut page_table,
        &mut UEFIFrameAllocator,
        false 
    ).expect("Fail to load and map kernel elf file");
    // FIXME: map kernel stack
    map_range(
        config.kernel_stack_address,
        match config.kernel_stack_auto_grow {
            0 => config.kernel_stack_size,
            _ => config.kernel_stack_auto_grow / 4096,
        },
        &mut page_table,
        &mut UEFIFrameAllocator,
    ).unwrap();

    //load apps...is it right to be here?    
    let apps = if config.load_apps {
        info!("Loading apps...");
        Some(load_apps())
    } else {
        info!("Skip loading apps...");
        None
    };

    // FIXME: recover write protect (Cr0)
    unsafe {
        Cr0::update(|cr0| cr0.insert(Cr0Flags::WRITE_PROTECT));
    }
    free_elf(elf);

    // 5. Pass system table to kernel
    let ptr = uefi::table::system_table_raw().expect("Failed to get system table");
    let system_table = ptr.cast::<core::ffi::c_void>();

    // 6. Exit boot and jump to ELF entry
    info!("Exiting boot services...");

    let mmap = unsafe { uefi::boot::exit_boot_services(MemoryType::LOADER_DATA) };
    // NOTE: alloc & log are no longer available

    // construct BootInfo
    let bootinfo = BootInfo {
        memory_map: mmap.entries().copied().collect(),
        physical_memory_offset: config.physical_memory_offset,
        system_table,
        loaded_apps: apps,
    };

    // align stack to 8 bytes
    let stacktop = config.kernel_stack_address + config.kernel_stack_size * 0x1000 - 8;
    jump_to_entry(&bootinfo, stacktop);
}
