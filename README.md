# libr-os
A hobby operating system written in rustlang

## Setup Instructions

### Windows

1. Install [rustup](https://rustup.rs/).
2. Use nightly rust by running `rustup override set nightly`.
3. Install Microsoft's Visual Studio with C++ components for the linker.
4. Install bootimage with `cargo install bootimage`.
5. Install [QEMU](https://www.qemu.org/).
6. Add qemu directory to PATH environment variable. By default: `C:/Program Files/qemu`.
7. Run `cargo run` to build and launch to OS.
