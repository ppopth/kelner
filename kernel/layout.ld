ENTRY(_start)

SECTIONS {
    /* XXX: 0x9000 is the loading address for the kernel image. Currently, we
     * cannot increase this to higher than 0x10000. The loaded image is not
     * correct. I do not know why. Maybe, I should spend time investigating it.
     * However, if you want to change this value, do not forget to change it
     * at `bootloader/disk.s` as well because currently I do not know how to
     * refactor it. */
    . = 0x9000;
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
