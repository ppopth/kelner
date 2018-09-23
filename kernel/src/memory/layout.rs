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

const LIST_SIZE: usize = 0x100;

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
    /// Create [MemoryLayout](MemoryLayout) from data starting at
    /// address 0x500.
    pub fn new() -> MemoryLayout {
        unsafe {
            // In our convention, the bootloader will store the memory map
            // at 0x508 and the number of entries in the map at 0x500.
            let len = *(0x500 as *const usize);
            let mut list = [None; LIST_SIZE];

            let list_buffer = 0x508 as *const u64;

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
