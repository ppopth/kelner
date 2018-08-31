bits 16

; Before reading or writing to the disk we need to check if our used
; extension is supported or not.
check_int13h_extension:
    mov ah, 0x41
    mov dl, [data.disk_number]
    mov bx, 0x55aa

    int 0x13
    jc .extension_notsupported
    cmp bx, 0xaa55
    jnz .extension_notsupported

    ret

.extension_notsupported:
    mov si, loader_msg.extension_notsupported
    jmp error

; Load the payload to the memory.
load:
    ; We have to check if we can use it before using it.
    call check_int13h_extension

    ; 512 must already divide the size of the payload because we already pad
    ; it with zeroes.
    mov ax, (payload_end - payload_start) / 512
    ; If the number of sectors is greater than 127, we cannot load it using
    ; some Phoenix BIOSes. So it is better for the developers to reduce the
    ; size of the payload instead.
    cmp ax, 127
    ja .payload_too_large
    mov [dap.number], ax

    mov ax, payload_start
    mov [dap.offset], ax

    mov ax, 0
    mov [dap.segment], ax

    ; start should be zero so this should not be a problem either.
    mov eax, (payload_start - start) / 512
    mov [dap.address], eax

    mov ah, 0x42
    mov dl, [data.disk_number]
    mov si, dap

    int 0x13
    jc .load_error

    ret

.payload_too_large:
    mov si, loader_msg.payload_too_large
    jmp error
.load_error:
    mov si, loader_msg.load_error
    jmp error

; Used with int 13h to read the disk
dap:
                 db 0x10
                 db 0
.number:         dw 0
.offset:         dw 0
.segment:        dw 0
.address:        dd 0
                 dd 0

loader_msg:
.payload_too_large:       db "The payload is too large. We cannot load it ", \
                             "using some BIOSes.", 0
.load_error:              db "Errors found while booting Kelner.", 0
.extension_notsupported:  db "INT 13h extension is not supported.", 0
