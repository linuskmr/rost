//! Utilities for printing to the vga buffer.
use core::{fmt, ptr};
use core::fmt::Write;
use core::mem::size_of;
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const VGA_BUFFER_ADDRESS: usize = 0xb8000;

lazy_static! {
    /// Global writer to the vga buffer.
    pub(crate) static ref WRITER: Mutex<Writer> = Mutex::new(Writer::default());
}

/// Print something to the vga_buffer.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

/// Print something to the vga_buffer and include a newline.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Internally used print function.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}

/// Sets the color of the Writer to `color_code`. All following chars will get printed with this color code,
/// until you set a new color code.
pub(crate) fn set_color_code(color_code: ColorCode) {
    WRITER.lock().set_color_code(color_code);
}

/// Returns the current color code with which chars get printed.
pub(crate) fn get_color_code() -> ColorCode {
    WRITER.lock().get_color_code()
}

/// Available colors for in the vga buffer.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xa,
    LightCyan = 0xb,
    LightRed = 0xc,
    Pink = 0xd,
    Yellow = 0xe,
    White = 0xf,
}

/// ColorCode is composed of a foreground [Color] and a background [Color].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct ColorCode(u8);

impl ColorCode {
    /// Creates a new ColorCode with a `foreground` and `background` color.
    pub(crate) fn new(foreground: Color, background: Color) -> Self {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

impl Default for ColorCode {
    fn default() -> Self {
        Self::new(Color::White, Color::Black)
    }
}

/// A char that will be displayed by the vga buffer. It consist of an ascii value and a color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_value: u8,
    color_code: ColorCode,
}

/// VgaBuffer represents the memory-mapped VGA buffer.
/// Because accessing (writing) to the buffer has side effects (updates the VGA buffer memory),
/// the compiler should not optimize away. Thus the elements are [Volatile].
type VgaBuffer = [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT];

/// Position of the cursor in a [Writer].
struct CursorPosition {
    y: usize,
    x: usize,
}

/// A helper to write to a [VgaBuffer].
pub(crate) struct Writer {
    /// Buffer to write to.
    buffer: &'static mut VgaBuffer,
    /// The current cursor position.
    cursor_position: CursorPosition,
    /// The color with which letters get printed.
    color_code: ColorCode,
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            match byte {
                // Printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // Not printable char
                _ => self.write_byte(0x7e),
            }
        }
        Ok(())
    }
}

impl Writer {
    /// Writes `byte` to the vga buffer with the current color. If the current line would overflow,
    /// a newline will be inserted.
    pub(crate) fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\r' => self.carriage_return(),
            byte => {
                if self.cursor_position.x >= BUFFER_WIDTH {
                    self.new_line();
                }
                let screen_char = ScreenChar {
                    ascii_value: byte,
                    color_code: self.color_code,
                };
                let CursorPosition { y, x } = self.cursor_position;
                self.buffer[y][x].write(screen_char);
                self.cursor_position.x += 1;
            }
        }
    }

    /// Adds a newline to the vga buffer. This is done by copying all lines one line up, which discards the topmost
    /// line. The bottommost line is cleared and the cursor performs a carriage return.
    fn new_line(&mut self) {
        if self.cursor_position.y == BUFFER_HEIGHT-1 {
            // Screen is full. Copy each char one line up
            for y in 1..BUFFER_HEIGHT {
                for x in 0..BUFFER_WIDTH {
                    let character = self.buffer[y][x].read();
                    self.buffer[y-1][x].write(character)
                }
            }
        } else {
            // Screen is not full yet. Write one line below
            self.cursor_position.y += 1;
        }

        self.clear_row(BUFFER_HEIGHT-1);
        self.carriage_return();
    }

    /// Performs a carriage return, i.e. sets the cursor back to the beginning of the line.
    fn carriage_return(&mut self) {
        self.cursor_position.x = 0;
    }

    /// Clears the row specified by `y`.
    fn clear_row(&mut self, y: usize) {
        let blank = ScreenChar {
            ascii_value: b'\0',
            color_code: ColorCode::default(),
        };
        for x in 0..BUFFER_WIDTH {
            self.buffer[y][x].write(blank);
        }
    }

    /// Sets the color of the Writer to `color_code`. All following chars will get printed with this color code,
    /// until you set a new color code.
    pub(crate) fn set_color_code(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }

    /// Returns the current color code with which chars get printed.
    pub(crate) fn get_color_code(&self) -> ColorCode {
        self.color_code
    }
}

impl Default for Writer {
    fn default() -> Self {
        Self {
            buffer: unsafe { &mut *(VGA_BUFFER_ADDRESS as *mut VgaBuffer) },
            cursor_position: CursorPosition { y: 0, x: 0 },
            color_code: Default::default(),
        }
    }
}