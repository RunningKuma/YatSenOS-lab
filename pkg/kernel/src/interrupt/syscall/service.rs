use core::alloc::Layout;

use crate::proc::*;
use crate::utils::*;

use super::SyscallArgs;


pub fn spawn_process(args: &SyscallArgs) -> usize {
    // FIXME: get app name by args
    //       - core::str::from_utf8_unchecked
    //       - core::slice::from_raw_parts
    // FIXME: spawn the process by name
    // FIXME: handle spawn error, return 0 if failed
    // FIXME: return pid as usize
    let name = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(args.arg0 as *const u8, args.arg1 as usize)) };
    let pid = spawn(name);
    if pid.is_none() {
        return 0;
    } else {
        pid.unwrap().0 as usize
    }
}

pub fn sys_write(args: &SyscallArgs) -> usize { //fixed
    // FIXME: get buffer and fd by args
    //       - core::slice::from_raw_parts
    let fd = args.arg0 as u8;
    let buf = unsafe { core::slice::from_raw_parts(args.arg1 as *const u8, args.arg2 as usize) };
    // FIXME: call proc::write -> isize 
    let count = write(fd, buf) as usize;
    // FIXME: return the result as usize
    count
}

pub fn sys_read(args: &SyscallArgs) -> usize { //fixed
    // FIXME: just like sys_write
    let fd = args.arg0 as u8;
    let buf = unsafe { core::slice::from_raw_parts_mut(args.arg1 as *mut u8, args.arg2 as usize) };
    let count = read(fd, buf) as usize;
    count
}

pub fn exit_process(args: &SyscallArgs,context:& mut ProcessContext) {
    // FIXME: exit process with retcode
    exit(args.arg0 as isize,context);
}

pub fn list_process() {
    // FIXME: list all processes
    print_process_list();
}

pub fn sys_allocate(args: &SyscallArgs) -> usize {
    let layout = unsafe { (args.arg0 as *const Layout).as_ref().unwrap() };

    if layout.size() == 0 {
        return 0;
    }

    let ret = crate::memory::user::USER_ALLOCATOR
        .lock()
        .allocate_first_fit(*layout);
    match ret {
        Ok(ptr) => ptr.as_ptr() as usize,
        Err(_) => 0,
    }
}

pub fn sys_deallocate(args: &SyscallArgs) {
    let layout = unsafe { (args.arg1 as *const Layout).as_ref().unwrap() };

    if args.arg0 == 0 || layout.size() == 0 {
        return;
    }

    let ptr = args.arg0 as *mut u8;

    unsafe {
        crate::memory::user::USER_ALLOCATOR
            .lock()
            .deallocate(core::ptr::NonNull::new_unchecked(ptr), *layout);
    }
}

pub fn sys_getpid() -> u16 {
        get_current_pid().0
}

pub fn sys_waitpid(args: &SyscallArgs) -> isize {
    let pid = ProcessId(args.arg0 as u16);
    let ret = get_return(pid);
    if !ret.is_none() {
        ret.unwrap() as isize
    } else {
        -1   
    }
        
}

pub fn sys_list_app() {
    list_app();
}

pub fn sys_list_proc(){
    list_process();
}