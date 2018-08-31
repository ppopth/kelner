bits 16

; Temporary global descriptor table.
gdt:
.null:  equ $ - gdt
        dq 0
.code:  equ $ - gdt
        dw 0          ; Limit (low)
        dw 0          ; Base (low)
        db 0          ; Base (middle)
        db 10011010b  ; A, R, C, DPL, P
        db 10101111b  ; Limit (high), AVL, D, G
        db 0          ; Base (high)
.data:  equ $ - gdt
        dw 0          ; Limit (low)
        dw 0          ; Base (low)
        db 0          ; Base (middle)
        db 10010010b  ; A, W, E, DPL, P
        db 11000000b  ; Limit (high), AVL, B, G
        db 0          ; Base (high)
.ptr:   dw $ - gdt - 1
        dq gdt

