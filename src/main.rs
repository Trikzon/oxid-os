#![no_std]
#![no_main]
#![feature(asm)]
#![allow(dead_code)]

use core::panic::PanicInfo;
use libr_os::{tty_println, vga};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    tty_println!("Hello World{}", "!");
    panic!("Forcing a crash...");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    libr_os::tty::TTY.lock().set_foreground_color(vga::Color::Red);
    tty_println!("{}", info);
    libr_os::tty::TTY.lock().set_foreground_color(vga::Color::White);
    loop {}
}
