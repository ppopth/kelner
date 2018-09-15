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
org 0x7c00
bits 16

%include "bootsector.asm"
%include "second_stage.asm"

; We already entered long mode. We need to use 64bit instructions instead.
bits 64

; Pad to make payload_start aligned with the sector
align 512
payload_start:

%defstr KERNEL_STR %[KERNEL_FILE]
incbin KERNEL_STR
hlt

; Pad to make payload_end aligned with the sector
align 512
payload_end:

