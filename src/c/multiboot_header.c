#include <stdint.h>

__attribute__((section(".multiboot"), used))
const uint32_t multiboot_header[8] = {
    0x1BADB002,       // magic
    0x00010003,       // flags: align modules, memory info, video info
    0xFFFFFFFF - (0x1BADB002 + 0x00010003) + 1, // checksum
    0,0,0,0,0         // reserved
};
