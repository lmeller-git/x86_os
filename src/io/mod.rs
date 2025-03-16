#![allow(dead_code)]
pub mod keyboard;
pub mod serial;
pub mod vga;
use core::fmt::{Arguments, Write};
pub use vga::Color;
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
    interrupts::without_interrupts(|| write!(vga::WRITER.lock(), "{}", args).unwrap())
}

pub fn _print_with_args(args: Arguments, bg: Color, fg: Color) {
    if let Some(s) = args.as_str() {
        for byte in s.bytes() {
            vga::WRITER.lock().write_byte(byte);
            vga::WRITER.lock().select_prev().unwrap().update(|c| {
                c.fg(fg).bg(bg);
            });
        }
    }
}
