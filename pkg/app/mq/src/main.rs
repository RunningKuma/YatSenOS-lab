

#![no_std]
#![no_main]

use lib::*;

extern crate lib;

const QUEUE_CAP_COUNT: usize = 1;
static mut COUNT: usize = 0;
static FULL: Semaphore = Semaphore::new(100);
static EMPTY: Semaphore = Semaphore::new(200);
static MUTEX: Semaphore = Semaphore::new(666);
static mut PIDS_PRODUCERS : [u16; 8] = [0; 8];  // 存储生产者的pid
static mut PIDS_CONSUMERS : [u16; 8] = [0; 8];  // 存储消费者的pid

fn main() -> isize {
    EMPTY.init(0);
    FULL.init(QUEUE_CAP_COUNT);
    MUTEX.init(1);
    for i in 0..16 {
        let pid = sys_fork();
        if pid == 0 {
            if i % 2 == 0 {
                unsafe { PIDS_PRODUCERS[i / 2] = sys_get_pid(); }
                producer();
            } else {
                unsafe { PIDS_CONSUMERS[i / 2] = sys_get_pid(); }
                consumer();
            }
        }
    }
    println!("All Finished, state of message queue:");
    sys_stat();

    for i in 0..8 {
        println!("Producer PID[{}]: {} ", i,unsafe{PIDS_PRODUCERS[i]});
        println!("Consumer PID[{}]: {} ", i, unsafe{PIDS_CONSUMERS[i]});
    }
    
    for i in 0..8 {
        println!("Waiting for Producer PID[{}]: {}", i, unsafe { PIDS_PRODUCERS[i] });
        sys_wait_pid( unsafe { PIDS_PRODUCERS[i] });
        println!("Waiting for Consumer PID[{}]: {}", i, unsafe { PIDS_CONSUMERS[i] });
        sys_wait_pid( unsafe { PIDS_CONSUMERS[i] });
    }

    MUTEX.free();
    EMPTY.free();
    FULL.free();
    
    println!("Final message count: {}", unsafe { COUNT });
    sys_exit(0);


}


fn producer() -> ! {
    let pid = sys_get_pid();
    println!("Producer {} started", pid);
    for _ in 0..10 {

        delay();
        FULL.wait();
        MUTEX.wait();
        unsafe {
            COUNT += 1;}
            println!("Producer {} produced message, total count: {}", pid, unsafe{COUNT} );
        
        MUTEX.signal();
        EMPTY.signal();
    }
    sys_exit(0);
}

fn consumer() -> ! {
    let pid = sys_get_pid();
    println!("Consumer {} started", pid);
    for _ in 0..10 {

        delay();
        EMPTY.wait();
        MUTEX.wait();
        unsafe {
            COUNT -= 1;}
            println!("Consumer {} consumed message, total count: {}", pid, unsafe{COUNT});

        MUTEX.signal();
        FULL.signal();
    }
    sys_exit(0);
}

#[inline(never)]
#[unsafe(no_mangle)]
fn delay() {
    for _ in 0..0x100 {
        core::hint::spin_loop();
    }
}

entry!(main);
