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
bits 64

; We will use LBA48 PIO mode here.
pio_load:
    ; This is an initial loading address. If you want to change this value,
    ; do not forget to change it at `kernel/layout.ld` as well because
    ; currently I do not know how to refactor it.
    mov rax, 0x100000
    mov [.load_address], rax
    ; 512 must already divide the size of the payload because we
    ; already pad it with zeroes.
    mov rax, (payload_end - payload_start) / 512
    mov [.sector_left], rax
    ; start should be zero so this should not be a problem either.
    mov rax, (payload_start - start) / 512
    mov [.sector_start], rax

.loop:
    ; Check if the number of sectors to be read is greater than 2 bytes.
    ; If so, we need to read the disk multiple times.
    mov rbx, [.sector_left]
    cmp rbx, 0xffff
    jbe .load
    mov rbx, 0xffff
.load:
    mov al, 0x40
    mov dx, 0x1f6
    out dx, al
    call wait_400ns
    call wait_bsy

    mov al, bh
    mov dx, 0x1f2
    out dx, al

    mov rax, [.sector_start]
    shr rax, 3 * 8
    mov dx, 0x1f3
    out dx, al

    mov rax, [.sector_start]
    shr rax, 4 * 8
    mov dx, 0x1f4
    out dx, al

    mov rax, [.sector_start]
    shr rax, 5 * 8
    mov dx, 0x1f5
    out dx, al

    mov al, bl
    mov dx, 0x1f2
    out dx, al

    mov rax, [.sector_start]
    shr rax, 0 * 8
    mov dx, 0x1f3
    out dx, al

    mov rax, [.sector_start]
    shr rax, 1 * 8
    mov dx, 0x1f4
    out dx, al

    mov rax, [.sector_start]
    shr rax, 2 * 8
    mov dx, 0x1f5
    out dx, al

    mov al, 0x24
    mov dx, 0x1f7
    out dx, al

    mov rdi, [.load_address]
.repl:
    call wait_400ns
    call wait_bsy
    mov dx, 0x1f7
    in al, dx
    ; Test if flag ERR or DF is set. If so, there is an error.
    test al, 0x21
    jnz .error
    ; Test DRQ. If set, we can read the data now.
    test al, 0x08
    jz .repl

    ; The data is ready now to be read.
    mov dx, 0x1f0
    ; Read one sector.
    mov rcx, 256
    rep insw
    call wait_400ns
    ; Check if we already read all sectors in this loop.
    dec rbx
    test rbx, rbx
    jnz .repl

    ; Reduce the number of sectors left.
    mov rbx, [.sector_left]
    cmp rbx, 0xffff
    jbe .recal
    mov rbx, 0xffff
.recal:
    mov rax, [.sector_left]
    sub rax, rbx
    mov [.sector_left], rax
    ; Increase the starting sector.
    mov rax, [.sector_start]
    add rax, rbx
    mov [.sector_start], rax
    ; Increase the load address.
    mov rcx, [.load_address]
    mov rax, rbx
    mov rsi, 512
    mul rsi
    add rax, rcx
    mov [.load_address], rax

    ; If there is no sector left, return.
    mov rax, [.sector_left]
    test rax, rax
    jnz .loop

    ret
.error:
    hlt
.load_address:  dq 0
.sector_left:   dq 0
.sector_start:  dq 0

wait_bsy:
    mov dx, 0x1f7
    in al, dx
    ; Check if BSY flag is set.
    test al, 0x80
    jnz wait_bsy
    ret

wait_400ns:
    mov dx, 0x1f7
    in al, dx
    in al, dx
    in al, dx
    in al, dx
    ret
