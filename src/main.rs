#![allow(clippy::all)]
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(libr_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use libr_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");

    libr_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    libr_os::test_panic_handler(info);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

