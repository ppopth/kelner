#![feature(panic_handler, start)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    hello();
    loop {}
}

fn hello() {
    let hello_str: &[u8] = b"Hello World!";
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in hello_str.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
