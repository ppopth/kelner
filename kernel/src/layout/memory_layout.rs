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

use core::slice;
use core::fmt;
use ::collections::{Interval, StaticIntvlist};

const LIST_SIZE: usize = 0x100;

// In our convention, the bootloader will store the memory map
// at 0x508 and the number of entries in the map at 0x500.
#[cfg(not(test))]
const LAYOUT_BUFFER: *const u64 = 0x500 as *const u64;

/// Memory entry type returned from the BIOS.
#[derive(Copy, Clone, Debug)]
enum MemoryEntryType {
    Free,
    Reserved,
    AcpiReclaimable,
    AcpiNvs,
    Bad,
}

/// Memory entry returned from the BIOS.
#[derive(Copy, Clone, Debug)]
struct MemoryEntry {
    base_address: usize,
    length: usize,
    entry_type: MemoryEntryType,
}

impl MemoryEntry {
    /// Create [MemoryEntry](MemoryEntry) from a slice.
    pub fn new(slice: &[u64]) -> MemoryEntry {
        MemoryEntry {
            base_address: slice[0] as usize,
            length: slice[1] as usize,
            entry_type: match slice[2] & 0xffff_ffff {
                1 => MemoryEntryType::Free,
                2 => MemoryEntryType::Reserved,
                3 => MemoryEntryType::AcpiReclaimable,
                4 => MemoryEntryType::AcpiNvs,
                5 => MemoryEntryType::Bad,
                _ => panic!("unknown memory entry type"),
            },
        }
    }
}

/// Memory layout which contains a list of [MemoryEntry](MemoryEntry).
pub struct MemoryLayout {
    list: [Option<MemoryEntry>; LIST_SIZE],
    len: usize,
}

impl MemoryLayout {
    /// Return [StaticIntvlist](StaticIntvlist) of all free memory entries.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn as_free_interval_list(&self) -> StaticIntvlist {
        let mut interval_list = StaticIntvlist::new();

        for item in self.list.iter().take(self.len) {
            let item = item.unwrap();
            if let MemoryEntryType::Free = item.entry_type {
                let interval = Interval::new(
                    item.base_address,
                    item.length,
                );
                interval_list.push(interval).unwrap();
            }
        }
        interval_list
    }

    /// Create [MemoryLayout](MemoryLayout) from data starting at
    /// address 0x500.
    pub fn new() -> MemoryLayout {
        unsafe {
            let len = *LAYOUT_BUFFER as usize;
            let mut list = [None; LIST_SIZE];
            let list_buffer = LAYOUT_BUFFER.offset(1);

            // Parse each entry in the map.
            for (i, item) in list.iter_mut().enumerate().take(len) {
                let slice = slice::from_raw_parts(
                    list_buffer.offset(i as isize * 3), 3
                );
                *item = Some(MemoryEntry::new(slice));
            }

            MemoryLayout {
                list,
                len,
            }
        }
    }
}

impl fmt::Debug for MemoryLayout {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("MemoryLayout")
            .field("list", &&self.list[..self.len])
            .finish()
    }
}

#[cfg(test)]
const LAYOUT_BUFFER: *const u64 = &[6_u64,
    0x0000_0000, 0x0009_fc00, 1,
    0x0009_fc00, 0x0000_0400, 2,
    0x000f_0000, 0x0001_0000, 2,
    0x0010_0000, 0x07ee_0000, 1,
    0x07fe_0000, 0x0002_0000, 2,
    0xfffc_0000, 0x0004_0000, 2] as *const u64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_correct_free_interval_list() {
        let memory_layout = MemoryLayout::new();
        let out_list = memory_layout.as_free_interval_list();

        let mut expected_list = StaticIntvlist::new();
        expected_list.push(Interval::new(0x0000_0000, 0x0009_fc00)).unwrap();
        expected_list.push(Interval::new(0x0010_0000, 0x07ee_0000)).unwrap();
        assert_eq!(out_list, expected_list);
    }
}
