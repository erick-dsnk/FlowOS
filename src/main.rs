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
#![test_runner(flow_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use flow_os::println;


// Main entry point called by the bootloader.
#[no_mangle] // disable mangling of the function name.
pub extern "C" fn _start() -> ! {
    println!("[*] Success!");

    flow_os::init();

    #[cfg(test)]
    test_main();

    println!("no crash");

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
    flow_os::test_panic_handler(info)
}
