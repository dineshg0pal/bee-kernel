#[unsafe(link_section = ".multiboot")]
#[unsafe(no_mangle)]
pub static MULTIBOOT_HEADER: [u32; 8] = [
    0xE85250D6, // magic
    0,          // architecture (i386)
    24,         // header length
    (0xE85250D6u32 + 0 + 24).wrapping_neg(), // checksum
    0,          // end tag type
    8,          // end tag size
    0,
    0,
];
