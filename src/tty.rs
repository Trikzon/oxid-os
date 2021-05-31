use core::fmt;
use crate::vga;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

lazy_static! {
    pub static ref TTY: Mutex<Tty> = Mutex::new(Tty {
        column: 0,
        char_color: vga::CharColor::new(vga::Color::White, vga::Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

type Buffer = [[Volatile<vga::Char>; VGA_WIDTH]; VGA_HEIGHT];

pub struct Tty {
    column: usize,
    char_color: vga::CharColor,
    buffer: &'static mut Buffer,
}

impl Tty {
    pub fn put_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column >= VGA_WIDTH {
                    self.new_line();
                }
                let row = VGA_HEIGHT - 1;
                let column = self.column;

                self.buffer[row][column].write(vga::Char::new(byte, self.char_color));
                self.column += 1;
            }
        }
    }

    pub fn put_str(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable IBM437 byte or newline
                0x20..=0x7e | b'\n' => self.put_byte(byte),
                // not part of the printable IBM437 range
                _ => self.put_byte(0xfe),
            }
        }
    }

    pub fn new_line(&mut self) {
        for row in 1..VGA_HEIGHT {
            for column in 0..VGA_WIDTH {
                let char = self.buffer[row][column].read();
                self.buffer[row - 1][column].write(char);
            }
        }
        self.clear_row(VGA_HEIGHT - 1);
        self.column = 0;
    }

    pub fn clear_row(&mut self, row: usize) {
        let blank_char = vga::Char::new(b' ', self.char_color);
        for column in 0..VGA_WIDTH {
            self.buffer[row][column].write(blank_char);
        }
    }

    pub fn clear(&mut self) {
        for row in 0..VGA_HEIGHT {
            self.clear_row(row);
        }
    }

    pub fn set_char_color(&mut self, char_color: vga::CharColor) {
        self.char_color = char_color;
    }
}

impl fmt::Write for Tty {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.put_str(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! tty_print {
    ($($arg:tt)*) => ($crate::tty::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! tty_println {
    () => ($crate::tty_print!("\n"));
    ($($arg:tt)*) => ($crate::tty_print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
#[deprecated]
macro_rules! char_color {
    () => ($crate::char_color!($crate::vga::CharColor::new(vga::Color::White, vga::Color::Black)));
    ($color:expr) => ($crate::tty::TTY.lock().set_char_color($color));
}

#[doc(hidden)]
#[inline]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    TTY.lock().write_fmt(args).unwrap();
}
