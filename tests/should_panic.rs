#![no_std]
#![no_main]

use libr_os::{QemuExitCode, exit_qemu, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();

    loop {}
}

use libr_os::serial_print;

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

