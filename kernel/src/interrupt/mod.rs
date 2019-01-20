// Copyright (c) 2019, Suphanat Chunhapanya
// This file is part of Kelner.
//
// Kelner is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.
//
// Kelner is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Kelner.  If not, see <https://www.gnu.org/licenses/>.

//! Interrupt module. This module handles all the interrupt things.

#[macro_use]
mod macros;

#[cfg(not(test))]
use ::syscall::interrupt_handler;
#[cfg(not(test))]
use ::util::set_bits;

#[cfg(not(test))]
static mut IDT: [u128; 0x100] = [0; 0x100];

/// Initialization function for interrupt module.
#[cfg(not(test))]
pub fn init() {
    // By the time I wrote this code, I'm not sure why I set .d to be 1.
    unsafe {
        // The interrupt handler for system calls.
        IDT[0x80] = idt_entry! {
          .offset = (interrupt_handler as usize) as u128,
          .selector = 1<<3,
          .d = 1, .dpl = 3, .p = 1
        };
        let base = IDT.as_ptr() as u64;
        let limit = 16 * IDT.len() as u16;
        asm!("sub $$80, %rsp
              mov %ax, (%rsp)
              mov %rbx, 2(%rsp)
              lidt (%rsp)
              add $$80, %rsp"
              :: "{ax}"(limit), "{rbx}"(base)
              : "memory"
              : "volatile");
    }
}
