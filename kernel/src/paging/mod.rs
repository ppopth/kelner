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

//! Paging module. This module contains functions and structures used in
//! paging mechanism.

#[macro_use]
mod macros;
mod context;

use config::PAGE_SIZE;
use self::context::PagingContext;

// XXX: We should query this number from CPUID instead.
const MAXPHYADDR: u8 = 52;

/// Make sure that the address is page aligned.
pub fn assert_align(addr: usize) {
    if addr & (PAGE_SIZE-1) != 0 {
        panic!("the address must be page aligned");
    }
}

/// Parse the virtual address to get indices of page directories and
/// page tables.
pub fn parse_addr(addr: usize) -> [usize; 4] {
    let mut result = [0; 4];
    assert!(addr >> 48 == 0);
    result[0] = (addr & ((1 << 48)-1)) >> 39;
    result[1] = (addr & ((1 << 39)-1)) >> 30;
    result[2] = (addr & ((1 << 30)-1)) >> 21;
    result[3] = (addr & ((1 << 21)-1)) >> 12;
    result
}

/// Initialization function for paging module.
pub fn init() {
    let _ = PagingContext::new();
}
