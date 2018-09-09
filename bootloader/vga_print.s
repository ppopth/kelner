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
bits 64

; Print a NUL-terminated string pointed by si with new line.
vga_println:
    call vga_print
    call vga_printnl
    ret

; Print a NUL-terminated string pointed by si.
vga_print:
    cld
.loop:
    lodsb
    test al, al
    jz .done
    call vga_printc
    jmp .loop
.done:
    ret

vga_printnl:
    mov bl, [vga_printc.cursor_y]
    inc bl
    cmp bl, 25
    jne .done
    mov bl, 0
.done:
    mov [vga_printc.cursor_y], bl
    mov bl, 0
    mov [vga_printc.cursor_x], bl
    ret

; Print a character at al.
vga_printc:
    mov cl, al
    xor rax, rax
    mov al, [.cursor_y]
    ; Multiply by 80 because we have 80 characters per line.
    mov rdx, 80
    mul rdx
    xor rbx, rbx
    mov bl, [.cursor_x]
    add rax, rbx
    mov rdx, 2
    mul rdx
    mov byte [rax + 0xb8000], cl
    mov byte [rax + 0xb8000 + 1], 0xf

    mov al, [.cursor_x]
    mov bl, [.cursor_y]
    inc al
    cmp al, 80
    jne .endif
    mov al, 0
    inc bl
    cmp bl, 25
    jne .endif
    mov bl, 0
.endif:

    mov [.cursor_x], al
    mov [.cursor_y], bl
    ret
.cursor_x:  db 0
.cursor_y:  db 0

