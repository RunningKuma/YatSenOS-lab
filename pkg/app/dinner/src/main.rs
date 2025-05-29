#![no_std]
#![no_main]

use lib::*;

extern crate lib;

fn main() -> isize{
    println!("Hello, YatSenOS!");

    
    // 这里可以添加更多的逻辑或调用其他函数
    // 例如，模拟一些操作或输出更多信息
    
    sys_exit(0) // 正常退出程序
}

// const PHILOSOPHER_COUNT: usize = 5;
// const EATING_CYCLES: usize = 3;

// // 5个筷子，每个筷子用信号量表示（初始值为1，表示可用）
// static CHOPSTICKS: [Semaphore; 5] = semaphore_array![0,1,2,3,4];

// fn main() -> isize {
    
//     // 初始化所有筷子
//     for i in 0..5 {
//         CHOPSTICKS[i].init(1);
//     }
    
//     let mut pids = [0u16; PHILOSOPHER_COUNT];
    
    
//     for i in 0..PHILOSOPHER_COUNT {
//         let pid = sys_fork();
//         if pid == 0 {
//             // 子进程：哲学家
//             philosopher(i);
//         } else {
//             // 父进程：记录子进程PID
//             pids[i] = pid;
//         }
//     }
    
//     sys_stat();
    
//     // 等待所有哲学家完成就餐
//     for i in 0..PHILOSOPHER_COUNT {
//         println!("Waiting for philosopher {} (PID: {}) to finish...", i, pids[i]);
//         sys_wait_pid(pids[i]);
//     }
    
//     // 清理资源
//     for i in 0..5 {
//         CHOPSTICKS[i].free();
//     }
    
//     sys_exit(0)
// }

// fn philosopher(id: usize) -> ! {
//     let pid = sys_get_pid();
//     println!("Philosopher {} (PID: {}) started thinking...", id, pid);
    
//     for cycle in 0..EATING_CYCLES {
//         println!("Philosopher {} (PID: {}) - Cycle {}/{}", id, pid, cycle + 1, EATING_CYCLES);
        
//         // 思考阶段
//         think(id);
        
//         // 尝试就餐
//         if try_to_eat(id) {
//             // 成功就餐
//             eat(id, cycle);
//         } else {
//             // 无法就餐（避免死锁的情况）
//             println!("Philosopher {} (PID: {}) couldn't eat, continuing to think...", id, pid);
//         }
        
//         // 添加随机延迟
//         random_delay(id);
//     }
    
//     println!("Philosopher {} (PID: {}) finished all {} cycles and is leaving", id, pid, EATING_CYCLES);
//     sys_exit(0);
// }

// fn think(id: usize) {
//     let pid = sys_get_pid();
//     println!("Philosopher {} (PID: {}) is thinking...", id, pid);
    
//     // 思考时间（使用延迟模拟）
//     for _ in 0..(id + 1) {
//         delay();
//     }
// }

// fn try_to_eat(id: usize) -> bool {
//     let left_chopstick = id;
//     let right_chopstick = (id + 1) % 5;
//     let pid = sys_get_pid();
    
//     println!("Philosopher {} (PID: {}) is hungry and trying to pick up chopsticks {} and {}", 
//              id, pid, left_chopstick, right_chopstick);
    
//     // 为了避免死锁，奇数号哲学家先拿右筷子，偶数号哲学家先拿左筷子
//     if id % 2 == 0 {
//         // 偶数号：先拿左筷子，再拿右筷子
//         println!("Philosopher {} (PID: {}) trying to pick up left chopstick {}", id, pid, left_chopstick);
//         CHOPSTICKS[left_chopstick].wait();
//         println!("Philosopher {} (PID: {}) picked up left chopstick {}", id, pid, left_chopstick);
        
//         // 添加小延迟增加竞争
//         delay();
        
//         println!("Philosopher {} (PID: {}) trying to pick up right chopstick {}", id, pid, right_chopstick);
//         CHOPSTICKS[right_chopstick].wait();
//         println!("Philosopher {} (PID: {}) picked up right chopstick {}", id, pid, right_chopstick);
//     } else {
//         // 奇数号：先拿右筷子，再拿左筷子
//         println!("Philosopher {} (PID: {}) trying to pick up right chopstick {}", id, pid, right_chopstick);
//         CHOPSTICKS[right_chopstick].wait();
//         println!("Philosopher {} (PID: {}) picked up right chopstick {}", id, pid, right_chopstick);
        
//         // 添加小延迟增加竞争
//         delay();
        
//         println!("Philosopher {} (PID: {}) trying to pick up left chopstick {}", id, pid, left_chopstick);
//         CHOPSTICKS[left_chopstick].wait();
//         println!("Philosopher {} (PID: {}) picked up left chopstick {}", id, pid, left_chopstick);
//     }
    
//     true
// }

// fn eat(id: usize, cycle: usize) {
//     let left_chopstick = id;
//     let right_chopstick = (id + 1) % 5;
//     let pid = sys_get_pid();
    
//     println!("Philosopher {} (PID: {}) is EATING (Cycle {}) with chopsticks {} and {}", 
//              id, pid, cycle + 1, left_chopstick, right_chopstick);
    
//     // 就餐时间
//     for _ in 0..3 {
//         delay();
//     }
    
//     println!("Philosopher {} (PID: {}) finished eating (Cycle {})", id, pid, cycle + 1);
    
//     // 放下筷子（释放资源）
//     println!("Philosopher {} (PID: {}) putting down chopstick {}", id, pid, left_chopstick);
//     CHOPSTICKS[left_chopstick].signal();
    
//     println!("Philosopher {} (PID: {}) putting down chopstick {}", id, pid, right_chopstick);
//     CHOPSTICKS[right_chopstick].signal();
    
//     println!("Philosopher {} (PID: {}) put down both chopsticks and returning to think", id, pid);
// }

// fn random_delay(id: usize) {
//     // 基于哲学家ID的伪随机延迟
//     let delay_cycles = (id * 17 + 7) % 5 + 1;
//     for _ in 0..delay_cycles {
//         delay();
//     }
// }

// #[inline(never)]
// #[unsafe(no_mangle)]
// fn delay() {
//     for _ in 0..0x100 {
//         core::hint::spin_loop();
//     }
// }

// entry!(main);
