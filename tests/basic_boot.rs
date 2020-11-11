#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(flow_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use flow_os::println;


/* ##################################### TESTS ##################################### */

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

/* ################################################################################# */



#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    flow_os::test_panic_handler(info)
}