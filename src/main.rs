#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    let vga = 0xb8000 as *mut u8;
    unsafe {
        *vga.offset(0) = b'B';
        *vga.offset(1) = 0x0f;
        *vga.offset(2) = b'e';
        *vga.offset(3) = 0x0f;
        *vga.offset(4) = b'e';
        *vga.offset(5) = 0x0f;
    }
    loop {}
}
