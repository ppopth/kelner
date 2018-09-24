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

use alloc::boxed::Box;
use alloc::collections::btree_map::BTreeMap;
use alloc::collections::btree_set::BTreeSet;
use ::config::IDENTITY_MAP_MEMORY;
use ::memory::IntervalList;
use ::paging::MAXPHYADDR;
use ::util::set_bits;

const NUMBER_OF_ENTRIES: usize = 2^9;

enum PageDirTab {
    Directory(Box<PageDirectory>),
    Table(Box<PageTable>),
}

/// A structure that contains a list of pages.
struct PageTable {
    // A set of base addresses of currently present pages.
    set: BTreeSet<usize>,
    // A blob that the processor will read as a page table.
    blob: Box<[u64; NUMBER_OF_ENTRIES]>,
}

/// A structure that contains a list of next page directories or page tables.
struct PageDirectory {
    // A map of addresses to next page directories or page tables.
    map: BTreeMap<usize, PageDirTab>,
    // A blob that the processor will read as a page directory.
    blob: Box<[u64; NUMBER_OF_ENTRIES]>,
}

/// A structure that represents the whole paging context.
pub struct PagingContext {
    // A value that will loaded to CR3 when this paging context is used.
    cr3: u64,
    // A root page directory.
    directory: PageDirectory,
}

impl PagingContext {
    /// Create a new [PagingContext](PagingContext).
    pub fn new() -> PagingContext {
        let directory = PageDirectory {
            map: BTreeMap::new(),
            blob: Box::new([0; NUMBER_OF_ENTRIES]),
        };
        let cr3 = cr3! {
            .address = &*directory.blob as *const _ as u64
        };
        let context = PagingContext {
            cr3,
            directory,
        };

        // Initialize identity map memory sections.
        let _intervals = IntervalList::from(IDENTITY_MAP_MEMORY);

        context
    }
}
