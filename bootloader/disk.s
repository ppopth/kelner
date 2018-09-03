; Copyright (c) 2018, Suphanat Chunhapanya
; This file is part of Kelner.
;
; Kelner is free software; you can redistribute it and/or
; modify it under the terms of the GNU General Public License
; as published by the Free Software Foundation; either version 2
; of the License, or (at your option) any later version.
;
; Kelner is distributed in the hope that it will be useful,
; but WITHOUT ANY WARRANTY; without even the implied warranty of
; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
; GNU General Public License for more details.
;
; You should have received a copy of the GNU General Public License
; along with Kelner.  If not, see <https://www.gnu.org/licenses/>.
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

; We already entered long mode. We need to use 64bit instructions instead.
bits 64

entry:
    cli
    mov ax, gdt.data
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

%ifdef KERNEL_FILE
%ifdef ENTRY_POINT
    jmp ENTRY_POINT
    ; XXX: 0x9000 is the loading address for the kernel image. Currently, we
    ; cannot increase this to higher than 0x10000. The loaded image is not
    ; correct. I do not know why. Maybe, I should spend time investigating it.
    ; However, if you want to change this value, do not forget to change it at
    ; `kernel/layout.ld` as well because currently I do not know how to
    ; refactor it.
    times 0x9000 - 0x7c00 - ($ - $$) db 0
    %defstr KERNEL_STR %[KERNEL_FILE]
    incbin KERNEL_STR
    hlt
%endif
%endif

    ; If KERNEL_FILE and ENTRY_POINT are not defined, this means that we build
    ; it in a wrong way. Let us do something here.

    ; Print `OKAY` to screen
    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax
    hlt

; Pad to make payload_end aligned with the sector
align 512
payload_end:

