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

