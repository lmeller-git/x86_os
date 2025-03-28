use futures_util::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
pub mod shell;
use crate::{io::keyboard::ScancodeStream, print, println, Result};

fn should_exit(buf: &[u8; 4]) -> Result<()> {
    if buf == b"exit" {
        println!("\nExiting...");
        Err(crate::KernelError::ExitSignal(0))
    } else {
        Ok(())
    }
}

fn exit(buf: &[u8; 4]) {
    if should_exit(buf).is_err() {
        crate::exit_qemu(crate::QemuExitCode::Success);
    }
}

pub async fn print_keypresses() {
    print!("> ");
    let mut buf: [u8; 4] = [0, 0, 0, 0];
    let mut i = 0;
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        if character == '\n' {
                            exit(&buf);
                            i = 0;
                            print!("\n> ");
                        } else if i < 4 {
                            buf[i] = character as u8;
                            i += 1;
                            print!("{}", character);
                        } else {
                            i = 0;
                            buf[i] = character as u8;
                            print!("{}", character);
                        }
                    }
                    DecodedKey::RawKey(key) => print!("{:?}", key),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_exit() -> Result<()> {
        let buf = b"exit";
        assert!(should_exit(buf).is_err_and(|e| e == crate::KernelError::ExitSignal(0)));
        let buf = b"1234";
        assert!(should_exit(buf).is_ok());
        Ok(())
    }
}
