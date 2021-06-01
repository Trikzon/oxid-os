use crate::vga;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

const DEFAULT_FOREGROUND_COLOR: vga::Color = vga::Color::White;
const DEFAULT_BACKGROUND_COLOR: vga::Color = vga::Color::Black;

lazy_static! {
    pub static ref TTY: Mutex<Tty> = Mutex::new(Tty {
        column: 0,
        char_color: vga::CharColor::new(DEFAULT_FOREGROUND_COLOR, DEFAULT_BACKGROUND_COLOR),
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
        let mut bytes = s.bytes();
        while let Some(byte) = bytes.next() {
            match byte {
                // printable IBM437 byte or newline
                0x20..=0x7e | b'\n' => self.put_byte(byte),
                // is color format prefix '§'
                0xc2 => {
                    if let Some(next) = bytes.next() {
                        if next == 0xa7 {
                            if let Some(color_code) = bytes.next() {
                                self.read_color_code(color_code);
                            }
                        }
                    }
                }
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

    pub fn set_foreground_color(&mut self, foreground_color: vga::Color) {
        self.char_color.set_foreground(foreground_color);
    }

    pub fn reset_foreground_color(&mut self) {
        self.set_foreground_color(DEFAULT_FOREGROUND_COLOR);
    }

    pub fn set_background_color(&mut self, background_color: vga::Color) {
        self.char_color.set_background(background_color);
        for row in 0..VGA_HEIGHT {
            for column in 0..VGA_WIDTH {
                let mut prev_char = self.buffer[row][column].read();
                prev_char.char_color.set_background(background_color);
                self.buffer[row][column].write(prev_char);
            }
        }
    }

    pub fn reset_background_color(&mut self) {
        self.set_background_color(DEFAULT_BACKGROUND_COLOR);
    }

    fn read_color_code(&mut self, color_code: u8) {
        match color_code {
            b'r' => self.reset_foreground_color(),
            b'0' => self.set_foreground_color(vga::Color::Black),
            b'1' => self.set_foreground_color(vga::Color::Blue),
            b'2' => self.set_foreground_color(vga::Color::Green),
            b'3' => self.set_foreground_color(vga::Color::Cyan),
            b'4' => self.set_foreground_color(vga::Color::Red),
            b'5' => self.set_foreground_color(vga::Color::Magenta),
            b'6' => self.set_foreground_color(vga::Color::Brown),
            b'7' => self.set_foreground_color(vga::Color::LightGray),
            b'8' => self.set_foreground_color(vga::Color::DarkGray),
            b'9' => self.set_foreground_color(vga::Color::LightBlue),
            b'a' | b'A' => self.set_foreground_color(vga::Color::LightGreen),
            b'b' | b'B' => self.set_foreground_color(vga::Color::LightCyan),
            b'c' | b'C' => self.set_foreground_color(vga::Color::LightRed),
            b'd' | b'D' => self.set_foreground_color(vga::Color::Pink),
            b'e' | b'E' => self.set_foreground_color(vga::Color::Yellow),
            b'f' | b'F' => self.set_foreground_color(vga::Color::White),
            _ => crate::tty_println!("\n[Error] Invalid color code."),
        }
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

#[doc(hidden)]
#[inline]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    TTY.lock().write_fmt(args).unwrap();
}

#[cfg(test)]
pub mod tests {
    #[test_case]
    fn test_tty_println_simple() {
        tty_println!("test_tty_println_simple output");
    }

    #[test_case]
    fn test_tty_println_many() {
        for _ in 0..200 {
            tty_println!("test_tty_println_many output");
        }
    }

    #[test_case]
    fn test_tty_println_output() {
        let s = "Some test string that fits on a single line.";
        tty_println!("{}", s);
        for (i, c) in s.chars().enumerate() {
            let screen_char = super::TTY.lock().buffer[super::VGA_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ibm437_character), c);
        }
    }
}
