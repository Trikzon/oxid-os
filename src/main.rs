#![no_std]
#![no_main]
#![feature(asm)]
#![allow(dead_code)]

use core::panic::PanicInfo;
use libr_os::{tty_println, vga};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    tty_println!("Hello World{}", "!");

    tty_println!("§0Black");
    tty_println!("§1Blue");
    tty_println!("§2Green");
    tty_println!("§3Cyan");
    tty_println!("§4Red");
    tty_println!("§5Magenta");
    tty_println!("§6Brown");
    tty_println!("§7LightGray");
    tty_println!("§8DarkGray");
    tty_println!("§9LightBlue");
    tty_println!("§aLightGreen");
    tty_println!("§BLightCyan");
    tty_println!("§cLightRed");
    tty_println!("§DPink");
    tty_println!("§eYellow");
    tty_println!("§FWhite");
    tty_println!("§rReset");
    tty_println!("§1Multi§2-§3Color§4!!!§r");

    panic!("Forcing a crash...");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // libr_os::tty::TTY.lock().set_foreground_color(vga::Color::Red);
    tty_println!("§4{}§r", info);
    // libr_os::tty::TTY.lock().set_foreground_color(vga::Color::White);
    loop {}
}
