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
#![cfg(not(test))]

use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use ::kalloc::CONTEXT;

/// Empty structure to used in Rust's `global_allocator` feature.
pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // We have to make sure that we already initialized the alloc module
        // before allocating any memory.
        match CONTEXT.as_mut().unwrap().alloc(layout) {
            Ok(addr) => addr,
            Err(_) => ptr::null_mut(),
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // If the memory is not already allocated, just silently return
        // from the function. We don't want to panic because this is a usual
        // situation that will happen so often.
        let _ = CONTEXT.as_mut().unwrap().dealloc(ptr, layout);
    }
}
