// Disable std lib
#![no_std]
// Tell Rust we dont want the normal entry point (Rust's real crt0 entrypoint is fn start)
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rhodes_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rhodes_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

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
    rhodes_os::test_panic_handler(info)
}
