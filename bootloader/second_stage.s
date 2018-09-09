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
bits 16

; Pad to make payload_start aligned with the sector
align 512
second_stage_start:
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
    jmp gdt.code:long_mode_entry

second_stage_msg:
.in_long_mode:  db "We are in long mode already :)", 0

%include "cpu.s"
%include "a20.s"
%include "paging.s"
%include "gdt.s"
%include "pio_loader.s"

; We already entered long mode. We need to use 64bit instructions instead.
bits 64
long_mode_entry:
    cli
    mov ax, gdt.data
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    mov si, second_stage_msg.in_long_mode
    call vga_println

    call pio_load
    jmp ENTRY_POINT

%include "vga_print.s"

; Pad to make payload_start aligned with the sector
align 512
second_stage_end:
