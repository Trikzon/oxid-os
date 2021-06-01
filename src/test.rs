use core::panic::PanicInfo;
use crate::{tty_print, tty_println};

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
    where
        T: Fn(),
{
    fn run(&self) {
        tty_print!("{}...\t", core::any::type_name::<T>());
        self();
        tty_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    tty_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    // TODO: exit_qemu
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    tty_println!("[failed]\n");
    tty_println!("Error: {}\n", info);
    // TODO: exit_qemu
    loop { }
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
