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
#![allow(dead_code)]

//! Memory layout module. This module contains information about each section
//! in the memory returned by the BIOS.

mod layout;
mod interval;

use self::layout::MemoryLayout;

static mut MEMORY_LAYOUT: Option<MemoryLayout> = None;

/// Initialization function for the memory layout module.
pub fn init() {
    unsafe {
        MEMORY_LAYOUT = Some(MemoryLayout::new());
    }
}
