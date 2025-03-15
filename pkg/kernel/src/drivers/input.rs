use crossbeam_queue::ArrayQueue;
use alloc::string::String;
use lazy_static::lazy_static;


type Key = char;

lazy_static! {
    static ref INPUT_BUF: ArrayQueue<Key> = ArrayQueue::new(128);
}

#[inline]
pub fn push_key(key: Key) {
    if INPUT_BUF.push(key).is_err() {
        warn!("Input buffer is full. Dropping key '{:?}'", key);
    }
}

#[inline]
pub fn try_pop_key() -> Option<Key> {
    INPUT_BUF.pop()
}

#[inline]
pub fn pop_key() -> Key {
    loop {
        if let Some(key) = try_pop_key() {
            return key;
        }
    }
}

#[inline]
pub fn backspace() {
    print!("\x08\x20\x08");
}

#[inline]
pub fn get_line() -> String {
    let mut line = String::with_capacity(128);
    loop {
        let key = pop_key();
        match key {
            '\r' => {
                print!("\n");
                break;
            }
            '\x08' | '\x7F' => {
                if !line.is_empty() {
                    line.pop();
                    backspace();
                }
            }
            _ => {
                line.push(key);
                print!("{}",key)
            }
        }
    }
    line
}
