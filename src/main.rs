
#![no_std] // Don't link the standard library. because it depends on the os-dependent libc
#![no_main] // Disable startup via crt0 (c runtime)

mod vga;

use core::fmt::Write;
use core::panic::PanicInfo;
use crate::vga::{Color, ColorCode, Writer};

fn main() {
    println!("Main function");
}

/// Entry point for this binary.
/// Overwrites the `_start` entry point with c calling conventions, for which the linker looks.
/// Function never returns, because who should catch that? Instead we may shut down the computer or something.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();
    panic!("main() function exited");
}

/// This function is called when a panic occurs. It never returns.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\nPANIC:\n{}", info);
    loop {}
}