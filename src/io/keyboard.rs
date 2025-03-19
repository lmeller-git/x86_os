use core::sync::atomic::{AtomicBool, Ordering};
use lazy_static::lazy_static;
use spin::Mutex;

pub struct KeyBoardBuffer {
    new_input: AtomicBool,
    char: Mutex<char>,
}

impl KeyBoardBuffer {
    pub fn get_next(&self) -> Option<char> {
        if self.new_input.load(Ordering::SeqCst) {
            let c = { *self.char.lock() };
            self.new_input.store(false, Ordering::SeqCst);
            Some(c)
        } else {
            None
        }
    }

    pub fn put_next(&self, ch: char) -> bool {
        if self.new_input.load(Ordering::SeqCst) {
            false
        } else {
            {
                *self.char.lock() = ch;
            }
            self.new_input.store(true, Ordering::SeqCst);
            true
        }
    }
}

pub struct HaltingKeyBoardBuffer {
    new_input: AtomicBool,
    char: Mutex<char>,
}

impl HaltingKeyBoardBuffer {
    pub fn get_next(&self) -> char {
        while !self.new_input.load(Ordering::SeqCst) {
            x86_64::instructions::hlt();
        }
        let c = { *self.char.lock() };
        self.new_input.store(false, Ordering::SeqCst);
        c
    }

    pub fn put_next(&self, ch: char) -> bool {
        if self.new_input.load(Ordering::SeqCst) {
            false
        } else {
            {
                *self.char.lock() = ch;
            }
            self.new_input.store(true, Ordering::SeqCst);
            true
        }
    }
}

pub struct LongKeyBoardBuffer {
    char: Mutex<alloc::collections::VecDeque<char>>,
}

impl LongKeyBoardBuffer {
    pub fn get_next(&self) -> char {
        loop {
            if !self.char.lock().is_empty() {
                break;
            }
            x86_64::instructions::hlt();
        }
        let c = {
            self.char
                .lock()
                .pop_front()
                .expect("char consumed by other thread, while this one tried to use it")
        };
        c
    }

    pub fn put_next(&self, ch: char) {
        self.char.lock().push_back(ch);
    }
}

lazy_static! {
    pub static ref KEYBOARD_BUFFER: LongKeyBoardBuffer = LongKeyBoardBuffer { char: spin::Mutex::new(alloc::collections::VecDeque::new()) } ;//{
        // new_input: false.into(),
        // char: Mutex::new(' ')
    // };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test_case]
    fn get_keyboard() -> Result<()> {
        let test_char = '_';
        _ = KEYBOARD_BUFFER.get_next();
        KEYBOARD_BUFFER.put_next(test_char);
        assert_eq!(KEYBOARD_BUFFER.get_next(), test_char);
        Ok(())
    }
}
