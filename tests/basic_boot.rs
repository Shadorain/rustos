#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rustos::println;
use core::panic::PanicInfo;

#[panic_handler]
fn panic (info: &PanicInfo) -> ! {
    rustos::test_panic_handler(info)
}

pub fn test_runner (tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[test_case]
fn test_println () {
    println!("test_println output");
}

#[no_mangle]
pub extern "C" fn _start () -> ! {
    test_main();

    loop {}
}
