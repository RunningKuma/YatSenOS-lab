mod context;
mod data;
mod manager;
mod paging;
mod pid;
mod process;
mod processor;
use alloc::sync::Arc;
use manager::*;
use process::*;
use processor::{get_pid, Processor};
use vm::ProcessVm;
use crate::memory::PAGE_SIZE;
mod vm;
use xmas_elf::{ElfFile, program};


use alloc::string::{String, ToString};
use alloc::vec::Vec;
pub use context::ProcessContext;
pub use paging::PageTableContext;
pub use data::ProcessData;
pub use pid::ProcessId;

use x86_64::structures::idt::PageFaultErrorCode;
use x86_64::VirtAddr;
pub const KERNEL_PID: ProcessId = ProcessId(1);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ProgramStatus {
    Running,
    Ready,
    Blocked,
    Dead,
}

/// init process manager
pub fn init(boot_info: &'static boot::BootInfo) {
    let proc_vm = ProcessVm::new(PageTableContext::new()).init_kernel_vm();


    trace!("Init kernel vm: {:#?}", proc_vm);
    //TODO:?should set ProcessData?
    let app_list = boot_info.loaded_apps.as_ref();
    
    // kernel process
    let kproc = { /* FIXME: create kernel process */
        Process::new("God".to_string(),None,Some(proc_vm),Some(ProcessData::new()))
     };
    manager::init(kproc, app_list);

    info!("Process Manager Initialized.");
    print_process_list();
    
}

pub fn switch(context: &mut ProcessContext) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        // FIXME: switch to the next process
        let manager = get_process_manager();
        //      - save current process's context
        manager.save_current(&context);
        //      - handle ready queue update
        manager.push_ready(get_pid());
        //      - restore next process's context
        manager.switch_next(context);
    });
}

// pub fn spawn_kernel_thread(entry: fn() -> !, name: String, data: Option<ProcessData>) -> ProcessId {
//     x86_64::instructions::interrupts::without_interrupts(|| {
//         let entry = VirtAddr::new(entry as usize as u64);
//         get_process_manager().spawn_kernel_thread(entry, name, data)
//     })
// }

pub fn print_process_list() {
    x86_64::instructions::interrupts::without_interrupts(|| {
        get_process_manager().print_process_list();
    })
}

pub fn env(key: &str) -> Option<String> {
    x86_64::instructions::interrupts::without_interrupts(|| {
        // FIXME: get current process's environment variable
        get_process_manager().current().read().env(key)
    })
}

pub fn process_exit(ret: isize) -> ! {
    x86_64::instructions::interrupts::without_interrupts(|| {
        get_process_manager().kill_current(ret);
    });

    loop {
        x86_64::instructions::hlt();
    }
}

pub fn handle_page_fault(addr: VirtAddr, err_code: PageFaultErrorCode) -> bool {
    x86_64::instructions::interrupts::without_interrupts(|| {
        get_process_manager().handle_page_fault(addr, err_code)
    })
}

pub fn get_return(p_pid: ProcessId) -> Option<isize>{
    x86_64::instructions::interrupts::without_interrupts(|| {
      let manager =  get_process_manager();
      manager.pid_return_code(p_pid)
    })
}

//custom functions

pub fn list_app() {
    x86_64::instructions::interrupts::without_interrupts(|| {
        let app_list = get_process_manager().app_list();
        if app_list.is_none() {
            println!("[!] No app found in list!");
            return;
        }

        let apps = app_list
            .unwrap()
            .iter()
            .map(|app| app.name.as_str())
            .collect::<Vec<&str>>()
            .join(" | ");

        // TODO: print more information like size, entry point, etc.


        println!("[+] App list: {}", apps);
        
    });
}

pub fn spawn(name: &str) -> Option<ProcessId> {
    let app = x86_64::instructions::interrupts::without_interrupts(|| {
        let app_list = get_process_manager().app_list()?;
        app_list.iter().find(|&app| app.name.eq(name))
    })?;

    elf_spawn(name.to_string(), &app.elf)
}

pub fn elf_spawn(name: String, elf: &ElfFile) -> Option<ProcessId> {
    let pid = x86_64::instructions::interrupts::without_interrupts(|| {
        let manager = get_process_manager();
        let process_name = name.to_lowercase();
        let parent = Arc::downgrade(&manager.current());
        let pid = manager.spawn(elf, name, Some(parent),None);

        debug!("Spawned process: {}#{}", process_name, pid);
        pid
    });

    Some(pid)
}

pub fn read(fd: u8, buf: &mut [u8]) -> isize {
    x86_64::instructions::interrupts::without_interrupts(|| get_process_manager().read(fd, buf))
}

pub fn write(fd: u8, buf: &[u8]) -> isize {
    x86_64::instructions::interrupts::without_interrupts(|| get_process_manager().write(fd, buf))
}

pub fn exit(ret: isize, context: &mut ProcessContext) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        let manager = get_process_manager();
        // FIXME: implement this for ProcessManager
        manager.kill_current(ret);
        manager.switch_next(context);
    })
}

#[inline]
pub fn still_alive(pid: ProcessId) -> bool {
    // x86_64::instructions::interrupts::without_interrupts(|| {
        // check if the process is still alive
        let manager = get_process_manager();
        if manager.get_process_status(pid) != ProgramStatus::Dead {
            true
        } else {
            false
        }
}

pub fn get_current_pid() -> ProcessId {
    x86_64::instructions::interrupts::without_interrupts(|| {
        // get current process id
        get_process_manager().current().pid()
    })
}
