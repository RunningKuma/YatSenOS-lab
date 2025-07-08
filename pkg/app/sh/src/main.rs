#![no_std]
#![no_main]

use lib::{string::String, vec::Vec, *};

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
    loop {
        print!("kuma@ysos [>] ");
        let root = String::from("/");
        let input = stdin().read_line();
        let cmd = input.split_whitespace().collect::<Vec<&str>>();
        match cmd[0] {
            "help" => {
                println!("{}", HELP);
            }
            "app" => {
                sys_list_app();
            }
            "ps" => {
                sys_stat();
            }
            "run" => {
                let app = cmd[1];
                let pid = sys_spawn(app);
                println!("The app {} has been spawn,pid: {}", app, pid);
                sys_wait_pid(pid);
            }
            "ls" => sys_list_dir(root.as_str()),
            "cat" => {
                let fd = sys_open(cmd[1].to_ascii_uppercase().as_str(), 1);

                if fd == 0 {
                    errln!("File not found or cannot open");
                    return -1;
                }

                let mut buf = vec![0; 0x4000];

                let size = sys_read(fd, &mut buf);

                if size.is_none() {
                    errln!("Cannot read file");
                    return -1;
                }

                let size = size.unwrap();
                if size == 0 {
                    errln!("File is empty or buffer is too small!");
                    return -1;
                }

                for i in 0..size {
                    print!("{}", buf[i] as char);
                }
                println!("");

                sys_close(fd);
            }
            "exit" => {
                println!("Thank you for using ysos!");
                break;
            }
            "\n" | "\r" => {
                continue;
            }
            _ => {
                if cmd[0].is_empty() {
                    println!();
                }
                println!(
                    "You said: {}, Command not found, type 'help' for more info",
                    input
                );
            }
        }
    }
    sys_exit(0);
}

entry!(main);
