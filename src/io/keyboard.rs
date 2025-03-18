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
            let c = self.char.lock();
            self.new_input.store(false, Ordering::SeqCst);
            Some(*c)
        } else {
            None
        }
    }

    pub fn put_next(&self, ch: char) -> bool {
        if self.new_input.load(Ordering::SeqCst) {
            false
        } else {
            let mut c = self.char.lock();
            *c = ch;
            self.new_input.store(true, Ordering::SeqCst);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test_case]
    fn get_keyboard() -> Result<()> {
        let test_char = '_';
        _ = KEYBOARD_BUFFER.get_next();
        assert!(KEYBOARD_BUFFER.put_next(test_char));
        assert_eq!(KEYBOARD_BUFFER.get_next().unwrap(), '_');
        Ok(())
    }
}
