use core::fmt::Display;

use alloc::{boxed::Box, collections::VecDeque, string::String, vec::Vec};
use futures_util::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};
use spin::Mutex;

use crate::{
    io::{self, keyboard::ScancodeStream, Writer, WriterBackend},
    print,
};

pub struct Cmd {
    pub cmd: String,
    pub args: Args,
}

#[derive(Clone)]
pub struct Args {
    pub positional: Vec<String>,
    pub named: Vec<NamedArg>,
}

impl From<String> for Cmd {
    fn from(value: String) -> Self {
        todo!()
    }
}

impl Display for Cmd {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

#[derive(Clone)]
pub struct NamedArg {
    name: String,
    val: String,
}

pub struct Parser<B>
where
    B: WriterBackend + 'static,
{
    hist: VecDeque<Cmd>,
    hist_position: usize,
    cursor_pos: usize,
    writer: Writer<B>,
}

impl<B> Parser<B>
where
    B: WriterBackend + 'static,
{
    pub async fn next(&mut self, instream: &mut ScancodeStream) -> Option<&Cmd> {
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
                            self.writer.write_byte_at(self.cursor_pos, character as u8);
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
                                self.debumb_cur();
                            }
                            KeyCode::ArrowRight => {
                                self.bump_cur();
                            }
                            KeyCode::Backspace => {
                                self.clear_one();
                            }
                            KeyCode::Tab => {}
                            _ => {}
                        },
                    }
                }
            }
        }
        None
    }

    pub fn get_prev(&mut self) -> Option<&Cmd> {
        if let Some(arg) = self.hist.get(self.hist_position + 1) {
            self.hist_position += 1;
            return Some(arg);
        }
        self.hist.back()
    }

    pub fn get_next(&mut self) -> Option<&Cmd> {
        if let Some(arg) = self.hist.get(self.hist_position - 1) {
            self.hist_position -= 1;
            return Some(arg);
        }
        self.hist.front()
    }

    pub fn new(backend: &'static Mutex<B>) -> Self {
        Self {
            hist: VecDeque::new(),
            hist_position: 0,
            cursor_pos: 0,
            writer: Writer::new(backend),
        }
    }

    fn clear_line(&mut self) {
        self.writer.clear_row()
    }
    fn clear_one(&mut self) {
        self.writer.clear_one();
    }
    fn bump_cur(&mut self) {
        self.writer.bumb_cur(&mut self.cursor_pos);
    }
    fn debumb_cur(&mut self) {
        self.writer.debumb_cur(&mut self.cursor_pos);
    }
}
