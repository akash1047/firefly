#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(firefly::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use firefly::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    const OS_NAME: &'static str = "firefly";

    println!("Hello from {OS_NAME}!!!");

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    firefly::test_panic_handler(info)
}
