use crate::{io, Result};
use core::fmt::Write;
use io::{read_one, vga};

pub fn run() -> Result<()> {
    let mut buf = [0, 0, 0, 0];
    loop {
        let mut i = 0;
        vga::WRITER.lock().write_str("\n> ").unwrap();
        loop {
            if let Some(key) = read_one() {
                vga::WRITER.lock().write_char(key as char).unwrap();
                if key == b'\n' {
                    break;
                }
                if i < 4 {
                    buf[i] = key;
                    i += 1;
                }
            }
        }
        should_exit(&buf)?
    }
}

fn should_exit(buf: &[u8; 4]) -> Result<()> {
    if buf == b"exit" {
        vga::WRITER.lock().write_str("\nExiting...\n").unwrap();
        Err(crate::KernelError::ExitSignal(0))
    } else {
        Ok(())
    }
}
