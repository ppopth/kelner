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
#![feature(lang_items)]
#![feature(alloc)]
#![feature(alloc_error_handler)]
#![feature(tool_lints)]
#![feature(panic_handler)]
#![feature(doc_cfg)]
#![feature(allocator_api)]
#![no_std]
#![cfg_attr(all(not(test), not(rustdoc)), no_main)]

// Lints that are allowed.
#![allow(clippy::explicit_iter_loop)]

mod kalloc;
mod collections;
mod config;
mod util;
mod debug;

#[cfg(not(test))]
use core::panic::PanicInfo;
#[cfg(not(test))]
use core::alloc::Layout;

#[cfg(test)]
#[macro_use]
extern crate std;
extern crate alloc;
extern crate rlibc;
extern crate siphasher;

/// Global allocator which will be used when there is a heap allocation.
#[cfg(not(test))]
#[global_allocator]
static ALLOCATOR: kalloc::Allocator = kalloc::Allocator;

/// An entry function when the kernel is booted.
#[allow(clippy::empty_loop)]
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    kalloc::init();
    debug::println("Hello World!");
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

#[allow(clippy::empty_loop)]
#[cfg(not(test))]
#[alloc_error_handler]
#[no_mangle]
pub fn error_handler(_layout: Layout) -> ! {
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
