#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(libr_os::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use libr_os::tty_println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    libr_os::test::test_panic_handler(info)
}

#[test_case]
fn test_tty_println() {
    tty_println!("test_tty_println output");
}
