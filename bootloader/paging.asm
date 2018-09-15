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

; Temporary page table.
pgt:
.pml4t:  equ 0x1000
.pdpt:   equ 0x2000
.pdt:    equ 0x3000
align 512

disable_paging:
    ; Clear the PG-bit, which is bit 31.
    mov eax, cr0
    and eax, 0x7fffffff
    mov cr0, eax
    ret

setup_paging:
    ; In fact, we probably do not have to disable paging before entering the
    ; long mode, but after thinking again I think it is better to disable it
    ; for safety.
    call disable_paging

    ; We will use 4-level paging but we will set PS Flag in pdt entry
    ; to make the pdt entry a huge page because we want a 2MB identity
    ; map for 512 pages.

    ; Make pmlt4t point to pdpt. We set the last two bits because it must be
    ; present and writable.
    mov eax, pgt.pdpt
    or eax, 0b11
    mov [pgt.pml4t], eax
    ; Make pdpt point to pdt.
    mov eax, pgt.pdt
    or eax, 0b11
    mov [pgt.pdpt], eax

    ; Configure all 512 pages.
    mov ecx, 0
.loop:
    ; The page size is 2MB.
    mov eax, 0x200000
    mul ecx
    ; Bit 7 is for a huge page.
    or eax, (1 << 7) | 0b11
    mov [pgt.pdt + ecx * 8], eax
    inc ecx
    cmp ecx, 512
    jne .loop

    ; Load the page table to cr3.
    mov eax, pgt.pml4t
    mov cr3, eax

    ; Set PAE bit in cr4.
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; Set LME bit in IA32_EFER.
    mov ecx, 0xc0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; Set PG bit and PE bit in cr0 to enable paging!
    mov eax, cr0
    or eax, 1 << 31 | 1
    mov cr0, eax

    ret
