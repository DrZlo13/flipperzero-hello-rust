//! Hello, Rust!
//! Prints "Hello, Rust! 🦀" to the console, waits for one second then exits.

#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[repr(C)]

pub struct Foo {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[panic_handler]
fn panic(_panic_info: &PanicInfo<'_>) -> ! {
    // Halt!
    loop {}
}

extern "C" {
    fn printf(fmt: *const u8, ...) -> i32;
    fn furi_delay_ms(ms: u32);
}

#[no_mangle]
pub extern "C" fn hello_rust_app(_p: *mut Foo) -> i32 {
    unsafe {
        printf(b"Hello, Rust! \xF0\x9F\xA6\x80\r\n\0".as_ptr());
        furi_delay_ms(1000);
    }

    0
}
