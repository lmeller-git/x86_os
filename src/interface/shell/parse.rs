use core::fmt::Display;

use alloc::{collections::VecDeque, string::String, vec::Vec};
use futures_util::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};

use crate::{io::keyboard::ScancodeStream, print};

pub struct Args {
    cmd: String,
    positional: Vec<String>,
    named: Vec<NamedArg>,
}

impl From<String> for Args {
    fn from(value: String) -> Self {
        todo!()
    }
}

impl Display for Args {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

pub struct NamedArg {
    name: String,
    val: String,
}

pub struct Parser {
    hist: VecDeque<Args>,
    position: usize,
    cursor_pos: usize,
}

impl Parser {
    pub async fn next(&mut self, instream: &mut ScancodeStream) -> Option<&Args> {
        let mut buffer = String::new();
        let mut tmp = None;
        let mut keyboard = Keyboard::new(
            ScancodeSet1::new(),
            layouts::Us104Key,
            HandleControl::Ignore,
        );

        while let Some(scancode) = instream.next().await {
            if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
                if let Some(key) = keyboard.process_keyevent(key_event) {
                    match key {
                        DecodedKey::Unicode(character) => {
                            if character == '\n' {
                                self.hist.push_front(buffer.into());
                                return self.hist.front();
                            }
                            buffer.push(character);
                        }
                        DecodedKey::RawKey(key) => match key {
                            KeyCode::ArrowUp => {
                                tmp = self.get_prev();
                                if let Some(tmp) = tmp {
                                    print!("{}", tmp);
                                }
                            }
                            KeyCode::ArrowDown => {
                                tmp = self.get_next();
                                if let Some(tmp) = tmp {
                                    print!("{}", tmp);
                                }
                            }
                            KeyCode::ArrowLeft => {
                                self.cursor_pos -= 1;
                            }
                            KeyCode::ArrowRight => {
                                self.cursor_pos += 1;
                            }
                            KeyCode::Backspace => {
                                todo!()
                            }
                            _ => {}
                        },
                    }
                }
            }
        }
        None
    }

    pub fn get_prev(&mut self) -> Option<&Args> {
        if let Some(arg) = self.hist.get(self.position + 1) {
            self.position += 1;
            return Some(arg);
        }
        self.hist.back()
    }

    pub fn get_next(&mut self) -> Option<&Args> {
        if let Some(arg) = self.hist.get(self.position - 1) {
            self.position -= 1;
            return Some(arg);
        }
        self.hist.front()
    }

    pub fn new() -> Self {
        Self {
            hist: VecDeque::new(),
            position: 0,
            cursor_pos: 0,
        }
    }

    fn clear_line(&mut self) {}
    fn clear_one(&mut self) {}
    fn bump_cur(&mut self) {}
    fn debumb_cur(&mut self) {}
}
