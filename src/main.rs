// Disable std lib
#![no_std]
// Tell Rust we dont want the normal entry point (Rust's real crt0 entrypoint is fn start)
#![no_main]

// Define our own panic_handler, std lib provides one, but we have to roll our own now!
use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

// Disable name mangling (required, linker needs to know where our fn is!).
// This is our entry point! We use never "!", as the entry point isn't called by a function, it's invoked directly by the OS/bootloader.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}
