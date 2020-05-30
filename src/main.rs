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
    // VGA Test!!!!!!!!
    // VGA Text Buffer is at 0xb8000 (memory-mapped I/O)
    // Cast the address to a raw pointer
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // light cyan
        }
    }

    loop {}
}
