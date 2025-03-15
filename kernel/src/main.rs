#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader_api::{entry_point, BootInfo};
use core::fmt::Write;
use core::panic::PanicInfo;
use io::read_one;
mod io;
mod vga;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    vga::WRITER.lock().write_str("Hello").unwrap();
    #[cfg(test)]
    test_main();
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
        exit(&buf);
    }
}

fn exit(buf: &[u8; 4]) {
    if buf == b"exit" {
        vga::WRITER.lock().write_str("\nExiting...\n").unwrap();
        exit_qemu(QemuExitCode::Success);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    // println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    vga::WRITER.lock().write_str("test").unwrap()
}
