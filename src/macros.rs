use core::fmt;
use crate::tty;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::macros::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! char_color {
    () => ($crate::char_color!($crate::vga::CharColor::new(vga::Color::White, vga::Color::Black)));
    ($color:expr) => ($crate::tty::TTY.lock().set_char_color($color));
}

#[doc(hidden)]
#[inline]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    tty::TTY.lock().write_fmt(args).unwrap();
}
