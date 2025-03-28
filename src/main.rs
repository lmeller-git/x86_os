#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use alloc::boxed::Box;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use io::keyboard::ScancodeStream;
use lib_os::hlt_loop;
use lib_os::io::PrintBuilder;
use task::executor::Executor;
use task::Task;
use thiserror::Error;

mod fs;
mod interface;
mod interrupt;
mod io;
mod lib_os;
mod mem;
mod task;
mod term;
extern crate alloc;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("VGA buffer initialized");

    print!("Initializing Interrupts...");
    interrupt::init();
    PrintBuilder::new(format_args!("    [Ok]\n"))
        .fg(io::Color::Green)
        .print();

    print!("Initializing Paging...");
    let (mut mapper, mut frame_allocator) = unsafe {
        mem::paging::init(boot_info.physical_memory_offset, &boot_info.memory_map)
            .expect("could not initialize paging")
    };
    PrintBuilder::new(format_args!("    [Ok]\n"))
        .fg(io::Color::Green)
        .print();

    print!("Initializing Heap...");
    mem::heap::init(&mut mapper, &mut frame_allocator).expect("heap initiialization failed");
    PrintBuilder::new(format_args!("    [Ok]\n"))
        .fg(io::Color::Green)
        .print();

    print!("Initializing file system...");
    fs::init();
    PrintBuilder::new(format_args!("    [Ok]\n"))
        .fg(io::Color::Green)
        .print();

    #[cfg(test)]
    test_main();

    let io: &'static mut ScancodeStream = Box::leak(Box::new(ScancodeStream::new()));
    let mut executor = Executor::new();
    // EXECUT.set(executor);
    // executor.spawn(Task::new(interface::print_keypresses()));
    executor.spawn(Task::new(interface::shell::run_shell(io)));
    PrintBuilder::new(format_args!("\nWelcome to x86_os\n\n"))
        .fg(io::Color::Green)
        .print();
    executor.run()
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("panicked at {}", info);
    // exit_qemu(QemuExitCode::Failed);
    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    PrintBuilder::new(format_args!("[err]\n"))
        .fg(io::Color::Red)
        .print();
    serial_println!("[err]");
    exit_qemu(QemuExitCode::Failed);
    hlt_loop()
}

pub type Result<T> = core::result::Result<T, KernelError>;

#[derive(Debug, PartialEq, PartialOrd, Clone, Error)]
pub enum KernelError {
    #[error("exit with signal {0}")]
    ExitSignal(u8),
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
        print!("{}   ", test.name());
        serial_print!("{}    ", test.name());
        match test.run() {
            Ok(_) => {
                PrintBuilder::new(format_args!("[ok]"))
                    .fg(crate::io::Color::Green)
                    .print();
                serial_print!("[ok]");
            }
            Err(e) => {
                PrintBuilder::new(format_args!("[err]\n{}", e))
                    .fg(crate::io::Color::Red)
                    .print();
                serial_print!("[err]\n{}", e);
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
    use super::*;
    use alloc::{boxed::Box, vec};

    #[test_case]
    fn test_heap() -> Result<()> {
        let heap_value_1 = Box::new(42);
        assert_eq!(*heap_value_1, 42);
        let v = vec![42, 42, 42];
        assert_eq!(v.as_slice(), &[42, 42, 42]);
        Ok(())
    }

    #[test_case]
    fn test_heap_reuse() -> Result<()> {
        for i in 0..mem::heap::HEAP_SIZE {
            let x = Box::new(i);
            assert_eq!(*x, i);
        }
        Ok(())
    }
}
