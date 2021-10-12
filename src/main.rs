
#![no_std] // Don't link the standard library. because it depends on the os-dependent libc
#![no_main] // Disable startup via crt0 (c runtime)

use core::panic::PanicInfo;

/// Entry point for this binary.
/// Overwrites the `_start` entry point with c calling conventions, for which the linker looks.
/// Function never returns, because who should catch that? Instead we may shut down the computer or something.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let text = b"Hello World";
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in text.into_iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// This function is called when a panic occurs. It never returns.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}