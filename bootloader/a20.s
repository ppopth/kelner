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

; Check if the a20 line is already enabled or not. Return 0 in ax if disabled,
; return 1 otherwise.
check_a20:
    push gs
    push fs
    push di
    push si

    cli

    ; If the a20 line is disabled, the address 0000:0500 must be the same
    ; as FFFF:0510. What we are going to do is to try to write some value to
    ; 0000:0500 and some different value to FFFF:0510 and read from both.
    ; If they are equal, the a20 line is disabled.

    mov ax, 0x0000
    mov fs, ax
    mov ax, 0xffff
    mov gs, ax
    mov di, 0x0500
    mov si, 0x0510

    ; Save the old values at those memory addresses to the stack.
    mov al, byte [fs:di]
    push ax
    mov al, byte [gs:si]
    push ax

    mov byte [fs:di], 0x00
    mov byte [gs:si], 0xff

    cmp byte [fs:di], 0xff

    ; Restore the old values at those memory addresses to the stack.
    pop ax
    mov byte [gs:si], al
    pop ax
    mov byte [fs:di], al

    mov ax, 0
    je .done
    mov ax, 1

.done:
    pop si
    pop di
    pop fs
    pop gs
    ret

; Try to enable the a20 line using various methods. Please note that the order
; of each method is important because we will do the least risky one first.
; Each method has a very complicated method. You do not have to understand
; what it does.
enable_a20:
    call check_a20
    cmp ax, 1
    je .already_enabled

    ; Try to enable using bios.
    call enable_a20_bios
    ; Ignoring return value.
    call check_a20
    cmp ax, 1
    je .enabled_using_bios

    ; Try to enable using keyboard.
    call enable_a20_keyboard
    call check_a20
    cmp ax, 1
    je .enabled_using_keyboard

    ; Try to enable using fast a20 gate.
    call enable_a20_fast
    call check_a20
    cmp ax, 1
    je .enabled_using_fast

    ; Fail and halt.
    mov si, a20_msg.cannot_enabled
    jmp error

.already_enabled:
    mov si, a20_msg.already_enabled
    call puts
    call putnl
    ret
.enabled_using_bios:
    mov si, a20_msg.enable_using_bios
    call puts
    call putnl
    ret
.enabled_using_keyboard:
    mov si, a20_msg.enable_using_keyboard
    call puts
    call putnl
    ret
.enabled_using_fast:
    mov si, a20_msg.enable_using_fast
    call puts
    call putnl
    ret

a20_msg:
.already_enabled:        db "A20 line is already enabled. We do not have ", \
                            "to do anything.", 0
.cannot_enabled:         db "A20 line cannot be enabled.", 0
.enable_using_bios:      db "A20 line is enabled using bios.", 0
.enable_using_keyboard:  db "A20 line is enabled using 8042 keyboard ", \
                            "controller.", 0
.enable_using_fast:      db "A20 line is enabled using fast A20 Gate.", 0

enable_a20_fast:
    in al, 0x92
    test al, 2
    jnz .done
    or al, 2
    and al, 0xfe
    out 0x92, al
.done:
    ret

enable_a20_keyboard:
    cli
    call    a20wait
    mov     al, 0xad
    out     0x64, al
    call    a20wait
    mov     al, 0xd0
    out     0x64, al
    call    a20wait2
    in      al, 0x60
    push    eax
    call    a20wait
    mov     al, 0xd1
    out     0x64, al
    call    a20wait
    pop     eax
    or      al, 2
    out     0x60, al
    call    a20wait
    mov     al, 0xae
    out     0x64, al
    call    a20wait
    sti
    ret
a20wait:
    in      al,0x64
    test    al,2
    jnz     a20wait
    ret
a20wait2:
    in      al,0x64
    test    al,1
    jz      a20wait2
    ret

enable_a20_bios:
    ; A20-Gate Support
    mov ax, 0x2403
    int 0x15
    jb .fail
    cmp ah, 0
    jnz .fail

    ; A20-Gate Status
    mov ax, 0x2402
    int 0x15
    jb .fail
    cmp ah, 0
    jnz .fail

    cmp al, 1
    jz .success

    ; A20-Gate Activated
    mov ax, 0x2401
    int 0x15
    jb .fail
    cmp ah, 0
    jnz .fail

.success:
    ; This does not mean that it really succeeds. We need to check the a20
    ; line again.
    mov ax, 1
    jmp .done
.fail:
    mov ax, 0
.done:
    ret
