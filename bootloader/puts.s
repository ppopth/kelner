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

; Print a NUL-terminated string pointed by si.
puts:
    cld
.loop:
    lodsb
    test al, al
    jz .done
    call putc
    jmp .loop
.done:
    ret

; Print a character at al.
putc:
    pusha
    mov ah, 0x0e
    int 0x10
    popa
    ret

; Print a space
putsp:
    pusha
    mov al, ' '
    call putc
    popa
    ret

; Print a new line character
putnl:
    pusha
    mov al, 0x0d
    call putc
    mov al, 0x0a
    call putc
    popa
    ret
