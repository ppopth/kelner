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

    call bios_load

    jmp second_stage_start

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
%include "bios_loader.s"

data:
.disk_number:    db 0

bootsector_msg:
.booting:        db "Booting Kelner from the MBR...", 0
.halted:         db "Halted!", 0

times 510 - ($ - $$) db 0

dw 0xaa55
