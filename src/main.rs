#![no_std]
#![no_main]
#![feature(asm, custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(libr_os::test::test_runner)]
#![allow(dead_code, unreachable_code)]

use core::panic::PanicInfo;
use libr_os::{log_debug, log_error, log_info, logln, tty_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    tty_println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    log_debug!("Hello");
    log_info!("This is some info.");
    log_error!("Oh no... an error occurred.");

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

    logln!("Test");
    logln!("Hmmmmmmmm");

    panic!("Forcing a crash...");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    tty_println!("§4{}§r", info);
    loop {}
}
