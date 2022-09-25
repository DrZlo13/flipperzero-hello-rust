//! Low-level bindings to Furi kernel

use core::ffi::{c_char, c_void};

use crate::opaque;

opaque!(FuriThreadId);

extern "C" {
    pub fn furi_crash(message: *const c_char) -> !;
    pub fn furi_delay_ms(msec: u32);
    pub fn furi_delay_us(usec: u32);
    pub fn furi_thread_get_current_id() -> *const FuriThreadId;
    pub fn furi_thread_get_name(thead_id: *const FuriThreadId) -> *const c_char;
    pub fn furi_thread_stdout_flush() -> i32;
    pub fn furi_thread_stdout_write(data: *const u8, size: usize) -> usize;
    pub fn furi_thread_yield();
    pub fn furi_record_open(name: *const c_char) -> *mut c_void;
    pub fn furi_record_close(name: *const c_char);
}
