use crate::{log, logln, qemu};
use core::panic::PanicInfo;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        log!("{}...\t", core::any::type_name::<T>());
        self();
        logln!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    logln!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    qemu::exit(qemu::ExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    logln!("[failed]\n");
    logln!("Error: {}\n", info);
    qemu::exit(qemu::ExitCode::Failure);
    loop {}
}
