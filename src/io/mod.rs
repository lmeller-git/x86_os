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
struct Buffer {
    fg: Color,
    bg: Color,
}

impl core::fmt::Write for Buffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut writer = vga::WRITER.lock();
        for byte in s.bytes() {
            writer.write_byte(byte);
            // Hack
            if byte == b'\n' {
                continue;
            }
            writer.select_prev().unwrap().update(|c| {
                c.fg(self.fg).bg(self.bg);
            });
        }
        Ok(())
    }
}

pub fn _print_with_args(args: Arguments, bg: Color, fg: Color) {
    let mut buffer = Buffer { fg, bg };
    buffer.write_fmt(args).expect("Failed to write to VGA");
}
