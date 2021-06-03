#![no_std]
#![no_main]

use core::panic::PanicInfo;
use libr_os::{log, logln, qemu};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    logln!("[test did not panic]");
    qemu::exit(qemu::ExitCode::Failure);
    loop {}
}

fn should_fail() {
    log!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    logln!("[ok]");
    qemu::exit(qemu::ExitCode::Success);
    loop {}
}
