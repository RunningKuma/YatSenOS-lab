#![no_std]
#![no_main]

use lib::*;
use rand_chacha::{ChaCha20Rng, rand_core::SeedableRng};

extern crate lib;

const PHILOSOPHER_EATEN: usize = 5;
const EATING_CYCLES: usize = 3;
static mut EATEN: [u32; 5] = [0; 5];

// 5个筷子，每个筷子用信号量表示（初始值为1，表示可用）
static CHOPSTICKS: [Semaphore; 5] = semaphore_array![0, 1, 2, 3, 4];

fn main() -> isize {
    // 初始化所有筷子
    for i in 0..5 {
        CHOPSTICKS[i].init(1);
    }

    let mut pids = [0u16; PHILOSOPHER_EATEN];
    println!(
        "Enter the number for different cases:\n
    1. Normal case (May cause deadlock)\n
    2. Starvation case\n
    3. Deadlock prevention case\n"
    );

    let input = stdin().read_line().trim().parse::<u32>();

    let rng = ChaCha20Rng::seed_from_u64(sys_get_pid() as u64);
    for i in 0..PHILOSOPHER_EATEN {
        let pid = sys_fork();
        if pid == 0 {
            match input {
                Ok(1) => normal_case(i),
                Ok(2) => starvation_case(i),
                Ok(3) => deadlock_prevention_case(i, rng.clone()),
                _ => {
                    println!("Invalid input, defaulting to Normal case.");
                    normal_case(i);
                }
            }
            sys_exit(0);
        } else {
            // 父进程：记录子进程PID
            pids[i] = pid;
        }
    }

    sys_stat();

    for i in 0..PHILOSOPHER_EATEN {
        println!(
            "Waiting for philosopher {} (PID: {}) to finish...",
            i, pids[i]
        );
        sys_wait_pid(pids[i]);
    }

    for i in 0..5 {
        CHOPSTICKS[i].free();
    }

    sys_exit(0)
}

fn normal_case(id: usize) {
    println!(
        "Philosopher {} (PID: {}) started thinking...",
        id,
        sys_get_pid()
    );

    for cycle in 0..EATING_CYCLES {
        println!(
            "Philosopher {} (PID: {}) - Cycle {}/{}",
            id,
            sys_get_pid(),
            cycle + 1,
            EATING_CYCLES
        );
        CHOPSTICKS[id].wait();
        CHOPSTICKS[(id + 1) % PHILOSOPHER_EATEN].wait();
        println!(
            "Philosopher {} (PID: {}) is EATING (Cycle {})",
            id,
            sys_get_pid(),
            cycle + 1
        );

        eat(id);

        CHOPSTICKS[(id + 1) % PHILOSOPHER_EATEN].signal();
        CHOPSTICKS[id].signal();
    }

    println!(
        "Philosopher {} (PID: {}) finished all {} cycles and is leaving",
        id,
        sys_get_pid(),
        EATING_CYCLES
    );
    sys_exit(0);
}

fn starvation_case(id: usize) {
    for cycle in 0..EATING_CYCLES {
        think(id * 100 + cycle);
        println!(
            "Philosopher {} (PID: {}) - Cycle {}/{}",
            id,
            sys_get_pid(),
            cycle + 1,
            EATING_CYCLES
        );

        if id != 4 {
            CHOPSTICKS[id].wait();
            println!(
                "Philosopher {} (PID: {}) picked up left chopstick {}",
                id,
                sys_get_pid(),
                id
            );
            CHOPSTICKS[(id + 1) % PHILOSOPHER_EATEN].wait();
        } else {
            sleep(1000);
            CHOPSTICKS[(id + 1) % PHILOSOPHER_EATEN].wait();
            println!(
                "Philosopher {} (PID: {}) should picked up right chopstick {}",
                id,
                sys_get_pid(),
                (id + 1) % PHILOSOPHER_EATEN
            );
            CHOPSTICKS[id].wait();
        }

        eat(id);

        CHOPSTICKS[(id + 1) % PHILOSOPHER_EATEN].signal();
        CHOPSTICKS[id].signal();
    }
}

fn deadlock_prevention_case(id: usize, rand: ChaCha20Rng) {
    let pid = sys_get_pid();
    println!("Philosopher {} (PID: {}) started thinking...", id, pid);

    for cycle in 0..EATING_CYCLES {
        println!(
            "Philosopher {} (PID: {}) - Cycle {}/{}",
            id,
            pid,
            cycle + 1,
            EATING_CYCLES
        );
        let think_time = 500;
        sleep(think_time); // 思考阶段

        if id % 2 == 0 {
            CHOPSTICKS[id].wait();
            CHOPSTICKS[(id + 1) % 5].wait();
        } else {
            CHOPSTICKS[(id + 1) % 5].wait();
            CHOPSTICKS[id].wait();
        }

        unsafe { eat(id) }

        CHOPSTICKS[(id + 1) % 5].signal();
        CHOPSTICKS[id].signal();
    }

    println!(
        "Philosopher {} (PID: {}) finished all {} cycles and is leaving",
        id, pid, EATING_CYCLES
    );
    sys_exit(0);
}

fn think(id: usize) {
    let pid = sys_get_pid();
    println!("Philosopher {} (PID: {}) is thinking...", id, pid);

    // 思考时间（使用延迟模拟）
    for _ in 0..(id + 1) {
        delay();
    }
}

fn eat(id: usize) {
    let left_chopstick = id;
    let right_chopstick = (id + 1) % 5;
    let pid = sys_get_pid();

    println!(
        "Philosopher {} (PID: {}) is EATING with chopsticks {} and {}",
        id, pid, left_chopstick, right_chopstick
    );

    delay();
}

fn sleep(id: u64) {
    for _ in 0..id {
        delay();
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

// #![no_std]
// #![no_main]

// use lib::*;

// extern crate lib;

// static CHOPSTICK: [Semaphore; 5] = semaphore_array![0, 1, 2, 3, 4];
// static WAITER: Semaphore = Semaphore::new(64);

// fn main() -> isize {
//     let mut pids = [0u16; 5];

//     // allow 4 philosophers to eat at the same time
//     WAITER.init(4);

//     for chop in &CHOPSTICK {
//         chop.init(1);
//     }

//     for (i, item) in pids.iter_mut().enumerate() {
//         let pid = sys_fork();
//         if pid == 0 {
//             philosopher(i);
//         } else {
//             *item = pid;
//         }
//     }

//     let cpid = sys_get_pid();

//     println!("#{} holds threads: {:?}", cpid, &pids);

//     sys_stat();

//     for pid in pids {
//         println!("#{} Waiting for #{}...", cpid, pid);
//         sys_wait_pid(pid);
//     }

//     sys_exit(0);
// }

// fn philosopher(id: usize) -> ! {
//     let pid = sys_get_pid();

//     for _ in 0..100 {
//         // thinking
//         println!("philosopher #{} ({}) is thinking...", id, pid);
//         delay();

//         // hungry
//         WAITER.wait();
//         CHOPSTICK[id].wait();
//         CHOPSTICK[(id + 1) % 5].wait();
//         println!("philosopher #{} ({}) is eating...", id, pid);
//         CHOPSTICK[(id + 1) % 5].signal();
//         CHOPSTICK[id].signal();
//         WAITER.signal();
//     }
//     sys_exit(0);
// }

// #[inline(never)]
// #[unsafe(no_mangle)]
// fn delay() {
//     for _ in 0..100 {
//         core::hint::spin_loop();
//     }
// }

// entry!(main);
