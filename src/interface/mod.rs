use crate::{io::keyboard::KEYBOARD_BUFFER, lib_os::io::PrintBuilder, print, println, Result};

pub fn run() -> Result<()> {
    PrintBuilder::new(format_args!("Welcome to x86_os\n"))
        .fg(crate::io::Color::Green)
        .bg(crate::io::Color::Black)
        .print();

    let mut buf = [0, 0, 0, 0];
    loop {
        let mut i = 0;
        print!("> ");
        loop {
            if let Some(k) = KEYBOARD_BUFFER.get_next() {
                print!("{}", k);
                if k == '\n' {
                    break;
                }
                if i < 4 {
                    buf[i] = k as u8;
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
