use crate::{io, print, println, Result};
use io::read_one;

pub fn run() -> Result<()> {
    let mut buf = [0, 0, 0, 0];
    loop {
        let mut i = 0;
        println!();
        print!("> ");
        loop {
            if let Some(key) = read_one() {
                print!("{}", key as char);
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
        println!("\nExiting...");
        Err(crate::KernelError::ExitSignal(0))
    } else {
        Ok(())
    }
}
