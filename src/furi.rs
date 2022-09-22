//! High-level bindings to Furi kernel

use crate::sys;
pub struct Stdout;

impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            if sys::furi::furi_thread_stdout_write(s.as_ptr(), s.len()) != s.len() {
                return Err(core::fmt::Error);
            }
        }

        Ok(())
    }
}

impl Stdout {
    pub fn flush(&mut self) -> core::fmt::Result {
        unsafe {
            if sys::furi::furi_thread_stdout_flush() != 0 {
                return Err(core::fmt::Error);
            }
        }

        Ok(())
    }
}

/// Puts the current thread to sleep for at least the specified amount of time.
pub fn sleep(duration: core::time::Duration) {
    unsafe {
        sys::furi::furi_delay_ms(duration.as_millis() as u32);
    }
}
