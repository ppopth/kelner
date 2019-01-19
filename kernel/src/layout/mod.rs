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

//! Memory layout module. This module contains information about each section
//! in the memory returned by the BIOS.

mod memory_layout;

pub use self::memory_layout::MemoryLayout;
use ::collections::StaticIntvlist;
use ::config::USED_KERNEL_MEMORY;

/// Initialization function for the memory layout module.
#[cfg_attr(test, allow(dead_code))]
pub fn init() {
    let memory_layout = MemoryLayout::new();
    let free_memory_list = memory_layout.as_free_interval_list();
    let used_memory_list = StaticIntvlist::from(USED_KERNEL_MEMORY).unwrap();

    if !used_memory_list.is_covered_by(&free_memory_list) {
        panic!("the memory layout is invalid this system cannot use Kelner");
    }
}
