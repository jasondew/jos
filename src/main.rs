#![no_std]
#![no_main]

use core::panic::PanicInfo;

static WELCOME: &[u8] = b"Welcome to jOS!";

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let buffer = 0xb8000 as *mut u8;

    for (num, &byte) in WELCOME.iter().enumerate() {
        unsafe {
            *buffer.offset(num as isize * 2) = byte;
            *buffer.offset(num as isize * 2 + 1) = 0x9;
        }
    }

    loop {}
}
