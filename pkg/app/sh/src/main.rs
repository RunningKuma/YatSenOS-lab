#![no_std]
#![no_main]

use lib::{vec::Vec, *};

extern crate lib;

use alloc;

const HELP: &str = r"
    Commands:
    help            Show help message
    app             List all apps
    ps              Show system status
    run <app>       Run an app
    exit           Exit the shell
    Thank you for using ysos!
    22307058 runningkuma
";
//make a bash-like shell
fn main() -> isize {
    

    loop{
        print!("kuma@ysos [>] ");
        let input = stdin().read_line();
        let cmd = input.split_whitespace().collect::<Vec<&str>>();
        match cmd[0] {
            "help" => {
                println!("{}",HELP);
            },
            "app" => {
                sys_list_app();
            },
            "ps" => {
                sys_stat();
            }
            "run" => {
                let app = cmd[1];
                let pid = sys_spawn(app);
                println!("The app {} has been spawn,pid: {}", app, pid);
                sys_wait_pid(pid);
            }
            // "hello" => {
            //     let pid = sys_spawn("hello");
            //     println!("The app hello has been spawn,pid: {}", pid);
            //     let e = sys_wait_pid(pid);
            //     println!("The app hello has been exit, pid: {}, exit code: {}", pid, e);
            //     sys_stat();
            // }
            // "fac" => {
            //     let pid = sys_spawn("factorial");
            //     println!("The app factorial has been spawn,pid: {}", pid);
            //     sys_wait_pid(pid);
            //     sys_stat();
            // }
            "exit" => {
                println!("Thank you for using ysos!");
                break;
            }
            "\n" | "\r"=> {
                continue;
            }
            _ => {
                if cmd[0].is_empty() {
                    println!();
                }
                println!("You said: {}, Command not found, type 'help' for more info", input);
            }
        }
    }
    sys_exit(0);
}

entry!(main);
