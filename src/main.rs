#![feature(panic_info_message)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![no_std] // Don't link the standard library. because it depends on the os-dependent libc
#![no_main] // Disable startup via crt0 (c runtime)

mod vga;


use core::panic::PanicInfo;
use crate::vga::{Color, ColorCode, set_color_code};

fn main() {
    rainbow_print("Hello Linus!");

    for (vga, chr) in core::iter::zip(
        (0xb8000..0xb8fa0).into_iter().step_by(2),
        [0x4c, 0x69, 0x6e, 0x75, 0x73, 0x13] // CP473
    ) {
        unsafe { (vga as *mut u8).write_volatile(chr) }
    }
}

fn rainbow_print(s: &str) {
    let mut color = Color::Black;
    for (i, c) in s.chars().enumerate() {
        let color = match i % 7 {
            0 => Color::Blue,
            1 => Color::Green,
            2 => Color::Cyan,
            3 => Color::Red,
            4 => Color::Magenta,
            5 => Color::Brown,
            6 => Color::LightGray,
            _ => unreachable!(),
        };
        set_color_code(ColorCode::new(color, Color::Black));
        print!("{}", c);
    }
}

/// Entry point for this binary.
/// Overwrites the `_start` entry point with c calling conventions, for which the linker looks.
/// Function never returns, because who should catch that? Instead we may shut down the computer or something.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("_start() [{:p}]", _start as *const());

    if cfg!(test) {
        // println!("tests [{:p}]", test_main as *const());
        // test_main()
    } else {
        println!("main() [{:p}]", main as *const());
        main();
    }

    panic!("_start(): Program ended");
}

/// This function is called when a panic occurs. It never returns.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    set_color_code(ColorCode::new(Color::White, Color::Red));
    println!("\nPANIC:");
    println!("at {}", info.location().unwrap());
    println!("{}", info.message().unwrap());
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for (i, test) in tests.into_iter().enumerate() {
        // println!("Running test {} at {:p}", i, *test as *const());
        test();
        println!("[OK]")
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// Exits QEMU with the given exit code.
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        const QEMU_ISA_DEBUG_EXIT_IOBASE: u16 = 0xf4;
        let mut port = Port::new(QEMU_ISA_DEBUG_EXIT_IOBASE);
        port.write(exit_code as u32);
    }
}