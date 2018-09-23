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

; Try to get information about the memory. After this step, the memory map
; will be stored at address 0x508 and the number of entry will be stored at
; 0x500. Each entry is 24 bytes long.
detect_memory:
    ; The number of entries will be kept in ebp.
    xor ebp, ebp
    mov di, 0x508

    xor ebx, ebx
    mov edx, 0x0534D4150
    ; 0xe820 is the command number.
    mov eax, 0xe820
    mov ecx, 24
    ; ACPI 3.0 Extended Attributes bitfield should be 1 by default.
    mov [es:di + 20], dword 1
    int 0x15
    ; After invoking the interrupt, the followings must be true. Otherwise,
    ; the interrupt fails.
    ;   1. EAX will be set to 0x534D4150.
    ;   2. the Carry flag will be clear.
    ;   3. EBX will be set to some non-zero value.
    jc .error
    ; The edx could be trashed by some BIOSes. Restore it.
    mov edx, 0x0534D4150
    cmp eax, edx
    jne .error
    test ebx, ebx
    jz .error

    jmp .after_loop
.loop:
    mov eax, 0xe820
    mov ecx, 24
    ; ACPI 3.0 Extended Attributes bitfield should be 1 by default.
    mov [es:di + 20], dword 1
    int 0x15
    ; If ebx does not reset to 0, the function will return with Carry set
    ; when you try to access the entry after the last valid entry.
    jc .finish
.after_loop:
    ; If the current entry size is zero, skip the current one.
    test cl, cl
    jz .skip
    ; If the current entry is ACPI 3.0 response check the extra bit.
    ; Otherwise, skip checking the bit.
    cmp cl, 20
    jbe .not_24
    ; Check if the "ignore this data" bit clear.
    test byte [es:di + 20], 1
    jz .skip
.not_24:
    ; Get lower dword of memory region length.
    mov ecx, [es:di + 8]
    ; Or it with upper dword to test for zero.
    or ecx, [es:di + 12]
    ; If the 64bit region length is zero, skip this entry.
    jz .skip

    ; Increment the number of entries.
    inc ebp
    ; Even if the entry size is 20, it is a good idea to move the pointer
    ; for 24 bytes so that it will be 64 bit aligned.
    add di, 24
.skip:
    test ebx, ebx
    jz .finish
    jmp .loop
.finish:
    ; Store the number of entries.
    mov [0x500], ebp
    xor ebp, ebp
    mov [0x504], ebp
    ret
.error:
    ; Fail and halt.
    mov si, memory_msg.error
    jmp error

memory_msg:
.error:  db "There is an error while analyzing memory layout.", 0
