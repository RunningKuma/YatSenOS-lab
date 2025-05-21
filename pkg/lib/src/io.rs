use crate::*;
use alloc::string::{String, ToString};
use alloc::vec;

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

impl Stdin {
    fn new() -> Self {
        Self
    }

    pub fn read_line(&self) -> String {
        // FIXME: allocate string
        let mut output = String::new();
        
        // FIXME: read from input buffer
        loop{
            let mut buf = [0u8; 100];
            let read_stat = sys_read(0, &mut buf);
            if read_stat.is_none(){
                continue
            } else {
                    for i in 0..read_stat.unwrap(){
                    let k = buf[i] as char;
                    match k{// FIXME: handle backspace / enter...
                        '\n' | '\r' => { //处理回车,返回line
                            stdout().write("\n");
                            return output;// FIXME: return string
                        }
                        '\x08' | '\x7F' => { //处理退格
                            if !output.is_empty() {
                                output.pop();
                                stdout().write("\x08\x20\x08");
                            }
                        }
                        _ => { //处理其他字符
                            output.push(k);
                            stdout().write(&k.to_string());
                        }
                    }
                    }

                }
            }

        }
        //       - maybe char by char?
    }

impl Stdout {
    fn new() -> Self {
        Self
    }

    pub fn write(&self, s: &str) {
        sys_write(1, s.as_bytes());
    }
}

impl Stderr {
    fn new() -> Self {
        Self
    }

    pub fn write(&self, s: &str) {
        sys_write(2, s.as_bytes());
    }
}

pub fn stdin() -> Stdin {
    Stdin::new()
}

pub fn stdout() -> Stdout {
    Stdout::new()
}

pub fn stderr() -> Stderr {
    Stderr::new()
}
