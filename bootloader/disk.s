org 0x7c00
bits 16

%include "bootsector.s"

; Pad to make payload_start aligned with the sector
align 512
payload_start:

    ; Check if long mode is supported.
    call check_long_mode

    ; Disable A20 for fun, before enabling it :)
    mov ax, 0x2400
    int 0x15
    ; Try to enable the a20 line.
    call enable_a20

    ; We will setup a page table, but this one will be temporary and just
    ; good enough to let us enter the long mode because we want to maintain
    ; the table in Rust not Assembly.
    call setup_paging
    ; After setuping paging, we are in the compatibility mode not long
    ; mode yet.


    ; To move from compatibility mode to long mode, we need to construct a gdt
    ; with a 64bit code segment entry inside and jump to that segment.
    lgdt [gdt.ptr]
    ; Entering long mode now!
    jmp gdt.code:entry

%include "cpu.s"
%include "a20.s"
%include "paging.s"
%include "gdt.s"

entry:
; We already entered long mode. We need to use 64bit instructions instead.
bits 64
    cli
    mov ax, gdt.data
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Print `OKAY` to screen
    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax
    hlt

; Pad to make payload_end aligned with the sector
align 512
payload_end:

