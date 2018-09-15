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

; Print a new line character
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

; Print address in rax
vga_printa:
    mov rbx, rax
    mov rcx, 16
.loop:
    dec rcx
    mov rax, rcx
    mov rdx, 4
    mul rdx
    mov rdx, rbx

    mov rsi, rcx
    mov rcx, rax
    shr rdx, cl
    mov rcx, rsi

    and dl, 0xf
    cmp dl, 10
    jb .small
    add dl, 'a'
    sub dl, 10
    jmp .endif
.small:
    add dl, '0'
.endif:

    push rcx
    push rbx
    mov al, dl
    call vga_printc
    pop rbx
    pop rcx
    test rcx, rcx
    jnz .loop

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

