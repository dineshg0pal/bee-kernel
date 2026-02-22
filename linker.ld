ENTRY(rust_main)

SECTIONS
{
    /* Place multiboot header at the very beginning */
    . = 0x100000;
    .multiboot : {
        KEEP(*(.multiboot))
    }

    .text : {
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

    .bss : {
        *(.bss*)
        *(COMMON)
    }

    /DISCARD/ : { *(.eh_frame) }
}
