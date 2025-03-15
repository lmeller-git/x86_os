#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::fmt;
use core::panic::PanicInfo;
mod interface;
mod io;
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    #[cfg(test)]
    test_main();
    loop {
        match interface::run() {
            Ok(_) => exit_qemu(QemuExitCode::Success),
            Err(e) => match e {
                KernelError::ExitSignal(s) => exit_qemu(QemuExitCode::Success),
            },
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

pub type Result<T> = core::result::Result<T, KernelError>;

#[derive(Debug)]
pub enum KernelError {
    ExitSignal(u8),
}

impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use core::fmt::Write;
    use io::vga;

    #[test_case]
    fn test_vga() {
        vga::WRITER.lock().write_str("test").unwrap()
    }
}
