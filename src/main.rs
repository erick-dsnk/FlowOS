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

use core::panic::PanicInfo;

mod vga_buffer;

// Main entry point called by the bootloader.
#[no_mangle] // disable mangling of the function name.
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;

    vga_buffer::WRITER.lock().write_string("It works!\n");

    loop {}
}

// Function called when the program panics.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[!] {}", info);

    loop {}
}
