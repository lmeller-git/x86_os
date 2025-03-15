use core::sync::atomic::{AtomicBool, Ordering};
use lazy_static::lazy_static;
use spin::Mutex;

pub struct KeyBoardBuffer {
    new_input: AtomicBool,
    char: Mutex<char>,
}

impl KeyBoardBuffer {
    pub fn get_next(&self) -> Option<char> {
        if self.new_input.load(Ordering::Relaxed) {
            let c = self.char.lock();
            self.new_input.store(false, Ordering::Relaxed);
            Some(*c)
        } else {
            None
        }
    }

    pub fn put_next(&self, ch: char) -> bool {
        if self.new_input.load(Ordering::Relaxed) {
            false
        } else {
            let mut c = self.char.lock();
            *c = ch;
            self.new_input.store(true, Ordering::Relaxed);
            true
        }
    }
}

lazy_static! {
    pub static ref KEYBOARD_BUFFER: KeyBoardBuffer = KeyBoardBuffer {
        new_input: false.into(),
        char: Mutex::new(' ')
    };
}
