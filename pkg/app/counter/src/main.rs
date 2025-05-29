#![no_std]
#![no_main]

use lib::*;

extern crate lib;

const THREAD_COUNT: usize = 8;
static mut COUNTER: isize = 0;
static LOCKER: SpinLock = SpinLock::new();
static SEMAPHORE: Semaphore = Semaphore::new(0);

fn main() -> isize {
    let pid = sys_fork();

    if pid == 0 {
        test_semaphore();
    } else {
        sys_wait_pid(pid);
        test_spin();
    }        
    sys_exit(0);
}

fn test_spin() -> isize {
    println!("test_spin() started...");
    let mut pids = [0u16; THREAD_COUNT];

    for i in 0..THREAD_COUNT {
        let pid = sys_fork();
        if pid == 0 {
            do_counter_inc();
            sys_exit(0);
        } else {
            pids[i] = pid; // only parent knows child's pid
        }
    }

    let cpid = sys_get_pid();
    println!("process #{} holds threads: {:?}", cpid, &pids);
    sys_stat();

    for i in 0..THREAD_COUNT {
        println!("#{} waiting for #{}...", cpid, pids[i]);
        sys_wait_pid(pids[i]);
    }

    println!("COUNTER result: {}", unsafe { COUNTER });
    //0       could not work!
    sys_exit(0)
}

fn do_counter_inc() {
    for _ in 0..100 {
        // FIXME: protect the critical section
        LOCKER.acquire();
        inc_counter();
        LOCKER.release();
    }
}

fn do_counter_inc_sem() {
    for _ in 0..100 {
        SEMAPHORE.wait();
        inc_counter();
        SEMAPHORE.signal();
    }
}

fn test_semaphore() -> isize {
    println!("test_semaphore() started...");
    let mut pids = [0u16; THREAD_COUNT];

    SEMAPHORE.init(1);
    for i in 0..THREAD_COUNT {
        let pid = sys_fork();
        if pid == 0 {
            do_counter_inc_sem();
            sys_exit(0);
        } else {
            pids[i] = pid; // only parent knows child's pid
        }
    }

    let cpid = sys_get_pid();
    println!("process #{} holds threads: {:?}", cpid, &pids);
    sys_stat();

    for i in 0..THREAD_COUNT {
        println!("#{} waiting for #{}...", cpid, pids[i]);
        sys_wait_pid(pids[i]);
    }

    println!("COUNTER result: {}", unsafe { COUNTER });
    //0       could not work!
    sys_exit(0)
}
/// Increment the counter
///
/// this function simulate a critical section by delay
/// DO NOT MODIFY THIS FUNCTION
fn inc_counter() {
    unsafe {
        delay();
        let mut val = COUNTER;
        delay();
        val += 1;
        delay();
        COUNTER = val;
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
fn delay() {
    for _ in 0..0x100 {
        core::hint::spin_loop();
    }
}

entry!(main);
