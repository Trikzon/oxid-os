#![no_std]
#![no_main]
#![feature(asm, custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod qemu;
pub mod serial;
pub mod test;
pub mod tty;
pub mod vga;

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop { }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test::test_panic_handler(info)
}

