/*
#![no_std]
    As this is an operating system, the comodities of the standard library can't be used
    since we need a program that can run on the bare minimum. We will have to implement
    everything from scratch.

#![no_main]
    We need to disable the default main entry point provided by Rust.
    The linker looks for a function named `_start` so we override it.
*/
#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;
mod serial;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11,
}

pub trait Testable {
    fn run(&self) -> ();
}


impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}


pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("\n[!] Running {} test(s)...", tests.len());

    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}


// Main entry point called by the bootloader.
#[no_mangle] // disable mangling of the function name.
pub extern "C" fn _start() -> ! {
    println!("[*] Success!");

    #[cfg(test)]
    test_main();

    loop {}
}

// Function called when the program panics in run mode.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[!] {}", info);

    loop {}
}


// Function called when the program panics in test mode.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[X] failed");
    serial_println!("  Error:\n    {}", info);

    exit_qemu(QemuExitCode::Failure);

    loop {}
}
