/*
As this is an operating system, the comodities of the standard library can't be used
since we need a program that can run on the bare minimum. We will have to implement
everything from scratch.
*/
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
