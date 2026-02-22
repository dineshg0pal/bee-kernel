#![no_std]

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

pub fn print(s: &str) {
    for (i, &byte) in s.as_bytes().iter().enumerate() {
        unsafe {
            *VGA_BUFFER.add(i * 2) = byte;
            *VGA_BUFFER.add(i * 2 + 1) = 0x0f; // white on black
        }
    }
}
