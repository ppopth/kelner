bits 16

start:
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov fs, ax
    mov gs, ax

    ; The stack should start below the code.
    mov sp, start

    ; Far jump to set cs.
    push ax
    push word .start
    retf

.start:
    ; save disk number
    mov [data.disk_number], dl

    ; Show initial booting message.
    mov si, bootsector_msg.booting
    call puts
    call putnl

    call load

    jmp payload_start

; Show the error message in si and halt.
error:
    call puts
    call putsp
    mov si, bootsector_msg.halted
    call puts
    call putnl
.halt:
    cli
    hlt
    jmp .halt

%include "puts.s"
%include "loader.s"

data:
.disk_number:    db 0

bootsector_msg:
.booting:        db "Booting Kelner from the MBR...", 0
.halted:         db "Halted!", 0

times 510 - ($ - $$) db 0

dw 0xaa55
