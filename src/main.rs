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

// Disable name mangling (required, linker needs to know where our fn is!).
// This is our entry point! We use never "!", as the entry point isn't called by a function, it's invoked directly by the OS/bootloader.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} {}",
        42,
        1.337
    )
    .unwrap();

    loop {}
}
