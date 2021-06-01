use x86_64::instructions::port::Port;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ExitCode {
    Success = 0x10, // (0x10 << 1) | 1 = 33
    Failure = 0x11, // (0x11 << 1) | 1 = 35
}

pub fn exit(exit_code: ExitCode) {
    let mut port = Port::new(0xf4);
    unsafe { port.write(exit_code as u32) };
}
