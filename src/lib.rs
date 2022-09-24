//! Hello, Rust!
//! Prints "Hello, Rust! 🦀" to the console, waits for one second then exits.

#![no_main]
#![no_std]

use core::fmt::Write;
use core::panic::PanicInfo;
use core::time::Duration;

use crate::furi::{Stdout, sleep};

mod furi;
mod sys;

#[panic_handler]
fn panic(panic_info: &PanicInfo<'_>) -> ! {
    let mut stdout = Stdout;

    write!(&mut stdout, "PANIC! {panic_info}\r\n").unwrap();
    stdout.flush().unwrap();

    // Halt!
    loop {}
}

/// Application entry point.
#[no_mangle]
pub extern "C" fn hello_rust_app(_args: *mut u8) -> i32 {
    let mut stdout = Stdout;

    write!(&mut stdout, "Hello, Rust! \u{1F980}\r\n").unwrap();
    stdout.flush().unwrap();

    sleep(Duration::from_secs(1));

    0
}
