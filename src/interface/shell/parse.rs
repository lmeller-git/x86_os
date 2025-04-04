use core::fmt::{Display, Write};

use alloc::{collections::VecDeque, string::String, vec::Vec};
use futures_util::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};
use spin::Mutex;

use crate::{
    io::{keyboard::ScancodeStream, Writer, WriterBackend},
    print,
};

pub struct Cmd {
    pub cmd: String,
    pub args: Args,
    pub out: Option<Redirect>,
    pub err: Option<Redirect>,
}

#[derive(Clone)]
pub struct Args {
    pub positional: Vec<String>,
    pub named: Vec<NamedArg>,
}

impl From<&str> for Args {
    fn from(value: &str) -> Self {
        if value.is_empty() {
            Self {
                positional: Vec::new(),
                named: Vec::new(),
            }
        } else {
            todo!()
        }
    }
}

impl From<&[&str]> for Args {
    fn from(value: &[&str]) -> Self {
        let mut v_iter = value.iter();
        let mut positional = Vec::new();
        let mut named = Vec::new();
        while let Some(&arg) = v_iter.next() {
            if arg.starts_with("-") {
                if let Some(&next) = v_iter.next() {
                    named.push((arg, next).into());
                }
            } else if !named.is_empty() {
                // TODO handle this error
                named.push((arg, "").into());
            } else {
                positional.push(arg.into());
            }
        }
        Self { positional, named }
    }
}

impl From<String> for Cmd {
    fn from(value: String) -> Self {
        if value.is_empty() {
            return Self {
                cmd: "".into(),
                args: "".into(),
                out: None,
                err: None,
            };
        }
        let unquoted = parse_quoted_str(&value);
        let cmd = (*unquoted.first().expect("impossible")).into();
        let mut out = None;
        let mut err = None;
        let mut j = unquoted.len();
        for (i, arg) in unquoted[1..].iter().enumerate() {
            match *arg {
                ">>" => {
                    if let Some(&output) = unquoted.get(i + 2) {
                        out = Some(Redirect::Append(output.into()))
                    }
                    j = j.min(i + 1);
                }
                "1>>" => {
                    if let Some(&output) = unquoted.get(i + 2) {
                        out = Some(Redirect::Append(output.into()))
                    }
                    j = j.min(i + 1);
                }
                "1>" => {
                    if let Some(&output) = unquoted.get(i + 2) {
                        out = Some(Redirect::Overwrite(output.into()))
                    }
                    j = j.min(i + 1);
                }
                "2>>" => {
                    if let Some(&output) = unquoted.get(i + 2) {
                        err = Some(Redirect::Append(output.into()))
                    }
                    j = j.min(i + 1);
                }
                "2>" => {
                    if let Some(&output) = unquoted.get(i + 2) {
                        err = Some(Redirect::Overwrite(output.into()))
                    }
                    j = j.min(i + 1);
                }
                _ => {}
            }
        }
        let args = unquoted[1..j].into();

        Self {
            cmd,
            args,
            out,
            err,
        }
    }
}

#[derive(Clone)]
pub enum Redirect {
    Append(String),
    Overwrite(String),
}

impl Display for Cmd {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

fn parse_quoted_str(data: &str) -> Vec<&str> {
    data.split(" ").collect()
}

#[derive(Clone)]
pub struct NamedArg {
    pub name: String,
    pub val: String,
}

impl From<(&str, &str)> for NamedArg {
    fn from(value: (&str, &str)) -> Self {
        let name = value.0.into();
        let val = value.1.into();
        Self { name, val }
    }
}

//TODO rework the entire cursor thing!! it is not needed here and can be handled by B
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
                        DecodedKey::Unicode('\u{08}') => {
                            self.clear_one(&mut buffer);
                        }
                        DecodedKey::Unicode(character) => {
                            write!(self.writer, "{}", character).unwrap();
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
                                // self.clear_one();
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
            cursor_pos: 2,
            writer: Writer::new(backend),
        }
    }

    fn clear_line(&mut self) {
        self.writer.clear_row()
    }
    fn clear_one(&mut self, buf: &mut String) {
        if !buf.is_empty() {
            buf.pop();
        }
        self.writer.clear_one();
        self.debumb_cur();
    }
    fn bump_cur(&mut self) {
        self.writer.bumb_cur(&mut self.cursor_pos);
    }
    fn debumb_cur(&mut self) {
        self.writer.debumb_cur(&mut self.cursor_pos);
    }
}
