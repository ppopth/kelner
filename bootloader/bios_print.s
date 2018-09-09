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

; Print a NUL-terminated string pointed by si with new line.
bios_println:
    call bios_print
    call bios_printnl
    ret

; Print a NUL-terminated string pointed by si.
bios_print:
    cld
.loop:
    lodsb
    test al, al
    jz .done
    call bios_printc
    jmp .loop
.done:
    ret

; Print a character at al.
bios_printc:
    pusha
    mov ah, 0x0e
    int 0x10
    popa
    ret

; Print a space
bios_printsp:
    pusha
    mov al, ' '
    call bios_printc
    popa
    ret

; Print a new line character
bios_printnl:
    pusha
    mov al, 0x0d
    call bios_printc
    mov al, 0x0a
    call bios_printc
    popa
    ret
