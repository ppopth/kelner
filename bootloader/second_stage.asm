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

    ; Analzy memory layout.
    call detect_memory

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
.in_long_mode:             db "We are in long mode already :)", 0
.kernel_out_of_range:      db "The kernel image is too large. ", \
                              "Please make it smaller.", 0
.sample_elf_out_of_range:  db "The sample ELF image is too large. ", \
                              "Please make it smaller.", 0
.payload_start:            db "payload_start ", 0
.payload_end:              db "payload_end   ", 0
.sample_elf_start:         db "sample_elf_start ", 0
.sample_elf_end:           db "sample_elf_end   ", 0
.bss_size:                 db "bss_size      ", 0

%include "cpu.asm"
%include "a20.asm"
%include "memory.asm"
%include "paging.asm"
%include "gdt.asm"
%include "pio_loader.asm"
%include "vga_print.asm"

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

    call load_kernel
    call load_sample_elf

    ; Set the stack pointer to the ending address of the kernel stack section.
    mov rsp, config.kernel_stack_end

    jmp ENTRY_POINT

load_kernel:
    ; Print payload_start
    mov si, second_stage_msg.payload_start
    call vga_print
    mov rax, config.kernel_code_start
    call vga_printa
    call vga_printnl

    ; Print payload_end
    mov si, second_stage_msg.payload_end
    call vga_print
    mov rax, config.kernel_code_start + (payload_end - payload_start)
    call vga_printa
    call vga_printnl

    ; Print BSS_SIZE
    mov si, second_stage_msg.bss_size
    call vga_print
    mov rax, BSS_SIZE
    call vga_printa
    call vga_printnl

    ; We need to add BSS_SIZE here because it isn't included in the
    ; payload yet.
    mov rax, config.kernel_code_start \
        + (BSS_SIZE + payload_end - payload_start)
    ; The range between config.kernel_code_start and config.kernel_code_end is
    ; for code and data, this means that if the payload_end is beyond
    ; config.kernel_code_end, it will go out of the range. We need to throw
    ; an error here.
    mov rbx, config.kernel_code_end
    cmp rax, rbx
    ja .out_of_range

    ; Before we can call pio_load, we need to put parameters in
    ; pio_load.load_address, pio_load.sector_left, and pio_load.sector_start
    ; first.
    mov rax, config.kernel_code_start
    mov [pio_load.load_address], rax
    ; 512 must already divide the size of the payload because we
    ; already pad it with zeroes.
    mov rax, (payload_end - payload_start) / 512
    mov [pio_load.sector_left], rax
    ; start should be zero so this should not be a problem either.
    mov rax, (payload_start - start) / 512
    mov [pio_load.sector_start], rax

    call pio_load

    ; Clear BSS section.
    xor rax, rax
    mov rdi, config.kernel_code_start + bss_start - payload_start
    mov rcx, BSS_SIZE
    rep stosb

    ret
.out_of_range:
    mov si, second_stage_msg.kernel_out_of_range
    call vga_println
    hlt

load_sample_elf:
    ; Print sample_elf_start
    mov si, second_stage_msg.sample_elf_start
    call vga_print
    mov rax, config.sample_elf_start
    call vga_printa
    call vga_printnl

    ; Print sample_elf_end
    mov si, second_stage_msg.sample_elf_end
    call vga_print
    mov rax, config.sample_elf_start + (sample_elf_end - sample_elf_start)
    call vga_printa
    call vga_printnl

    mov rax, config.sample_elf_start + (sample_elf_end - sample_elf_start)
    mov rbx, config.sample_elf_end
    cmp rax, rbx
    ja .out_of_range

    ; Put parameters for pio_load.
    mov rax, config.sample_elf_start
    mov [pio_load.load_address], rax
    mov rax, (sample_elf_end - sample_elf_start) / 512
    mov [pio_load.sector_left], rax
    mov rax, (sample_elf_start - start) / 512
    mov [pio_load.sector_start], rax

    call pio_load

    ret
.out_of_range:
    mov si, second_stage_msg.sample_elf_out_of_range
    call vga_println
    hlt

; Pad to make payload_start aligned with the sector
align 512
second_stage_end:
