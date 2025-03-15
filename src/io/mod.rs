#![allow(dead_code)]
pub mod keyboard;
pub mod vga;
use core::fmt::{Arguments, Write};
use x86_64::instructions::interrupts;

// TODO implement Bridge pattern for Writer Backend and possibly multiple Backends (how to handle Chars, Rows, Colors?)
pub trait WriterBackend: core::fmt::Write {
    // ...
    fn newline(&mut self) {}
    fn write_byte(&mut self, _byte: u8) {}
    fn write_bytes(&mut self, _bytes: &[u8]) {}
}

pub struct Writer<B: WriterBackend> {
    backend: B,
}

pub fn _print(args: Arguments) {
    interrupts::without_interrupts(|| vga::WRITER.lock().write_fmt(args).unwrap())
}
