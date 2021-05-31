#![no_std]
#![no_main]
#![feature(asm)]
#![allow(dead_code)]

mod tty;
mod vga;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    tty_println!("Hello World{}", "!");
    panic!("Forcing a crash...");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    char_color!(vga::CharColor::new(vga::Color::Red, vga::Color::Black));
    tty_println!("{}", info);
    char_color!();
    loop {}
}
