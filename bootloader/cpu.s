bits 16

; Check if the cpu supports long mode. This will check three things.
; If all are true, it does support.
; 1. Can we flip the ID Flag of EFLAGS?
; 2. Can we use CPUID(0x80000001)?
; 3. Is the EMT64T/LM Flag set?
check_long_mode:
    pushfd
    pop eax
    mov ecx, eax
    ; Try to flip the 21st bit (ID Flag) of EFLAGS to see if the cpu supports
    ; CPUID.
    xor eax, 1 << 21
    ; Store a new register value to EFLAGS.
    push eax
    popfd
    ; Read EFLAGS and will see later if the ID Flag changes or not.
    pushfd
    pop eax
    ; Store the old register value back to EFLAGS.
    push ecx
    popfd

    ; Compare eax (the new EFLAGS) and ecx (the old EFLAGS). If they are equal,
    ; this means the bit cannot be flipped and CPUID is not supported.
    xor eax, ecx
    jz cpuid_notsupported

    mov eax, 0x80000000
    cpuid
    ; Check whether extended function 0x80000001 is available are not.
    cmp eax, 0x80000001
    jb extended_cpuid_notsupported

    ; Check whether EMT64T/LM Flag set.
    ; In the old Intel architecture, this is called EMT64T.
    ; In AMD architecture, this is called LM.
    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29
    jz long_mode_notsupported

    ret

cpuid_notsupported:
    mov si, cpu_msg.cpuid_notsupported
    jmp error

extended_cpuid_notsupported:
    mov si, cpu_msg.extended_cpuid_notsupported
    jmp error

long_mode_notsupported:
    mov si, cpu_msg.long_mode_notsupported
    jmp error

cpu_msg:
.cpuid_notsupported:          db "CPUID is not supported.", 0
.extended_cpuid_notsupported:  db "Extened CPUID is not supported.", 0
.long_mode_notsupported:       db "Long mode is not supported.", 0
