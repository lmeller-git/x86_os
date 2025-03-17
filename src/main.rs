#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use bootloader::{entry_point, BootInfo};
use core::fmt;
use core::panic::PanicInfo;
use lib_os::hlt_loop;
mod interface;
mod interrupt;
mod io;
mod lib_os;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    println!("Initializing Interrupts...");
    interrupt::init();
    #[cfg(test)]
    test_main();
    match interface::run() {
        Ok(_) => exit_qemu(QemuExitCode::Success),
        Err(e) => match e {
            KernelError::ExitSignal(_s) => exit_qemu(QemuExitCode::Success),
        },
    }
    hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("panicked at {}", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop()
}

pub type Result<T> = core::result::Result<T, KernelError>;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum KernelError {
    ExitSignal(u8),
}

impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExitSignal(s) => write!(f, "Exit Signal {} recorded", s)?,
        }
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

pub trait Testable {
    fn name(&self) -> &str;
    fn run(&self) -> Result<()>;
}

impl<T> Testable for T
where
    T: Fn() -> Result<()>,
{
    fn name(&self) -> &str {
        core::any::type_name::<T>()
    }
    fn run(&self) -> Result<()> {
        self()
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    use lib_os::io::PrintBuilder;
    let mut failed = false;
    println!("Running {} tests", tests.len());
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        match test.run() {
            Ok(_) => {
                print!("{}   ", test.name());
                PrintBuilder::new(format_args!("[ok]"))
                    .fg(crate::io::Color::Green)
                    .print();
                serial_print!("{}   [ok]", test.name());
            }
            Err(e) => {
                print!("{}   ", test.name());
                PrintBuilder::new(format_args!("[err]\n{}", e))
                    .fg(crate::io::Color::Red)
                    .print();
                serial_print!("{}   [err]\n{}", test.name(), e);
                failed = true;
            }
        }
        println!();
        serial_println!();
    }
    // hlt_loop();
    if failed {
        exit_qemu(QemuExitCode::Failed);
    }
    exit_qemu(QemuExitCode::Success);
}

#[cfg(test)]
mod tests {
    // use super::*;
}
