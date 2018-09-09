ENTRY(_start)

SECTIONS {
    /* If you want to change this value, do not forget to change it
     * at `bootloader/pio_loader.s` as well because currently I do not
     * know how to refactor it. */
    . = 0x100000;
    .text ALIGN (0x1000) :
    {
        *(.text)
    }
    .rodata ALIGN (0x1000) :
    {
        *(.rodata)
    }
    .data ALIGN (0x1000) :
    {
        *(.data)
    }
    .bss ALIGN (0x1000) :
    {
        *(.bss)
    }
}
