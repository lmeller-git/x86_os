#![allow(dead_code)]
pub mod keyboard;
pub mod serial;
pub mod vga;
use core::fmt::{Arguments, Write};
use spin::Mutex;
pub use vga::Color;
use vga::VGAWriter;
use x86_64::instructions::interrupts;

// TODO implement Bridge pattern for Writer Backend and possibly multiple Backends (how to handle Chars, Rows, Colors?)
pub trait WriterBackend: core::fmt::Write {
    // ...
    fn newline(&mut self) {}
    fn write_byte(&mut self, _byte: u8) {}
    fn write_bytes(&mut self, _bytes: &[u8]) {}
    fn clear_one(&mut self) {}
    fn clear_row(&mut self) {}
    fn select_row(&mut self, row: usize) {}
    fn select_current_row(&mut self) {}
    fn select_current(&mut self) {}
    fn select_prev(&mut self) {}
    fn select_next(&mut self) {}
    fn write_byte_at(&mut self, pos: usize, byte: u8) {}
    fn delete_byte_at(&mut self, pos: usize) {}
    fn bumb_cur(&mut self, cur: &mut usize) {
        *cur += 1;
    }
    fn debumb_cur(&mut self, cur: &mut usize) {
        *cur -= 1;
    }
}

pub struct Writer<B: WriterBackend + 'static> {
    backend: &'static Mutex<B>,
}

impl<B> Writer<B>
where
    B: WriterBackend,
{
    pub fn new(backend: &'static Mutex<B>) -> Self {
        Self { backend }
    }
    pub fn newline(&mut self) {}
    pub fn write_byte(&mut self, _byte: u8) {}
    pub fn write_bytes(&mut self, _bytes: &[u8]) {}
    pub fn clear_one(&mut self) {
        self.backend.lock().clear_one();
    }
    pub fn clear_row(&mut self) {}
    pub fn select_row(&mut self, row: usize) {}
    pub fn select_current_row(&mut self) {}
    pub fn select_current(&mut self) {}
    pub fn select_prev(&mut self) {}
    pub fn select_next(&mut self) {}
    pub fn write_byte_at(&mut self, pos: usize, byte: u8) {
        self.backend.lock().write_byte_at(pos, byte)
    }
    pub fn delete_byte_at(&mut self, pos: usize) {}
    pub fn bumb_cur(&mut self, cur: &mut usize) {
        self.backend.lock().bumb_cur(cur);
    }
    pub fn debumb_cur(&mut self, cur: &mut usize) {
        self.backend.lock().debumb_cur(cur);
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test_case]
    fn test_print() -> Result<()> {
        _print(format_args!(""));
        Ok(())
    }

    #[test_case]
    fn test_print_args() -> Result<()> {
        _print_with_args(format_args!(""), Color::Black, Color::White);
        Ok(())
    }
}
