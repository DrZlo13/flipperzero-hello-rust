//! Low-level bindings to Furi kernel

extern "C" {
    pub fn furi_thread_stdout_flush() -> i32;
    pub fn furi_thread_stdout_write(data: *const u8, size: usize) -> usize;
    pub fn furi_delay_ms(ms: u32);
}
