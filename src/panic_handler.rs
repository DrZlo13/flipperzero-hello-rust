//! Panic handler for Furi applications.
//! This will print the panic info to stdout and then trigger a crash.

use core::ffi::{CStr, c_char};
use core::fmt::Write;
use core::panic::PanicInfo;
use core::str;

use crate::furi::Stdout;
use crate::sys::furi;

#[panic_handler]
fn panic(panic_info: &PanicInfo<'_>) -> ! {
    let mut stdout = Stdout;

    let thread_name = unsafe {
        let thread_id = furi::furi_thread_get_current_id();
        let thread_name = furi::furi_thread_get_name(thread_id);

        if thread_name.is_null() {
            "<unknown>"
        } else {
            str::from_utf8_unchecked(CStr::from_ptr(thread_name).to_bytes())
        }
    };

    // Format: "thread: 'App Name' paniced at 'panic!', panic.rs:5"
    let _ = write!(&mut stdout, "\x1b[0;31mthread '{thread_name}' {panic_info}\x1b[0m\r\n");
    let _ = stdout.flush();

    unsafe {
        furi::furi_thread_yield(); // Allow console to flush
        furi::furi_crash("Rust panic\r\n".as_ptr() as *const c_char)
    }
}
