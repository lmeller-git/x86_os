use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    sync::atomic::{AtomicBool, Ordering},
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use futures_util::Stream;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::{println, task::WAKER};

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
    pub static ref KEYBOARD_BUFFER: LongKeyBoardBuffer = LongKeyBoardBuffer {
        char: spin::Mutex::new(alloc::collections::VecDeque::new())
    };
}

// async
static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

pub fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if queue.push(scancode).is_err() {
            println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}

pub struct ScancodeStream;

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        ScancodeStream
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("scancode queue not initialized");

        if let Some(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(cx.waker());
        match queue.pop() {
            Some(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            None => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test_case]
    fn get_keyboard() -> Result<()> {
        let test_char = '_';
        // _ = KEYBOARD_BUFFER.get_next();
        KEYBOARD_BUFFER.put_next(test_char);
        assert_eq!(KEYBOARD_BUFFER.get_next(), test_char);
        KEYBOARD_BUFFER.put_next(test_char);
        KEYBOARD_BUFFER.put_next('4');
        assert_eq!(KEYBOARD_BUFFER.get_next(), test_char);
        assert_eq!(KEYBOARD_BUFFER.get_next(), '4');
        Ok(())
    }
}
