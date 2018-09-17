// Copyright (c) 2018, Suphanat Chunhapanya
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
#![feature(lang_items, tool_lints, panic_handler, doc_cfg)]
#![no_std]
#![cfg_attr(all(not(test), not(rustdoc)), no_main)]

// Lints that are allowed.
#![allow(clippy::explicit_iter_loop)]

#[cfg(not(test))]
mod alloc;
mod collections;
mod config;
mod util;
mod debug;

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(test)]
#[macro_use]
extern crate std;
extern crate rlibc;
extern crate siphasher;

/// Global allocator which will be used when there is a heap allocation.
#[cfg(not(test))]
#[global_allocator]
static ALLOCATOR: alloc::Allocator = alloc::Allocator;

/// An entry function when the kernel is booted.
#[allow(clippy::empty_loop)]
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    alloc::init();
    loop {}
}

/// A function that will be called when there is a panic.
#[allow(clippy::empty_loop)]
#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Mock function for Rust stack unwinding.
#[lang = "eh_personality"]
#[cfg(not(test))]
#[no_mangle]
pub extern fn eh_personality() {}

/// Mock function for libunwind's _Unwind_Resume.
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() {}
