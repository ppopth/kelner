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
