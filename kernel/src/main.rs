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
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(doc_cfg)]
#![feature(lang_items)]
#![feature(panic_handler)]
#![feature(panic_info_message)]
#![feature(tool_lints)]
#![no_std]
#![cfg_attr(all(not(test), not(rustdoc)), no_main)]

// Lints that are allowed.
#![allow(clippy::explicit_iter_loop)]

mod collections;
mod config;
#[cfg(not(test))]
#[macro_use]
mod debug;
mod kalloc;
mod layout;
mod paging;
mod util;

#[cfg(not(test))]
use core::alloc::Layout;
#[cfg(not(test))]
use core::panic::PanicInfo;

extern crate alloc;
extern crate rlibc;
extern crate siphasher;
#[cfg(test)]
#[macro_use]
extern crate std;

/// Global allocator which will be used when there is a heap allocation.
#[cfg(not(test))]
#[global_allocator]
static ALLOCATOR: kalloc::Allocator = kalloc::Allocator;

/// An entry function when the kernel is booted.
#[allow(clippy::empty_loop)]
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Booting...");
    init();
    println!("Hello World!");
    println!("My name is Kelner.");
    loop {}
}

#[cfg(not(test))]
/// Initialize everything.
fn init() {
    // Layout init must come before the kalloc init because kalloc uses
    // so much stack memory and layout init can check and abort if there is
    // not enough physical memory.
    layout::init();
    kalloc::init();
    paging::init();
}

/// A function that will be called when there is a panic.
#[allow(clippy::empty_loop)]
#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    debug::set_color(debug::Color::LightRed);
    println!("Kelner paniked!");
    if let Some(message) = info.message() {
        println!("{}", message);
    }
    if let Some(location) = info.location() {
        println!("panic occurred in file '{}' at line {}", location.file(),
            location.line());
    }
    debug::reset_color();
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
