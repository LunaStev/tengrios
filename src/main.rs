#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tengrios::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use tengrios::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    tengrios::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    tengrios::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    tengrios::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    tengrios::test_panic_handler(info)
}