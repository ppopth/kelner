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
use ::config::IDENTITY_MAP_MEMORY;
use ::memory::IntervalList;
#[cfg(not(test))]
use ::paging::MAXPHYADDR;
use ::paging::{assert_align, parse_addr};
#[cfg(not(test))]
use ::util::set_bits;

const NUMBER_OF_ENTRIES: usize = 2^9;

#[derive(Debug)]
enum PageDirTab {
    Directory(Box<PageDirectory>),
    Table(Box<PageTable>),
}

use self::PageDirTab::*;

/// A structure that contains a list of pages.
#[derive(Debug)]
struct PageTable {
    // A map of indices to the physical addresses.
    map: BTreeMap<usize, usize>,
    // A blob that the processor will read as a page table.
    blob: Box<[u64; NUMBER_OF_ENTRIES]>,
}

/// A structure that contains a list of next page directories or page tables.
#[derive(Debug)]
struct PageDirectory {
    // A map of indices to the next page directories or page tables.
    map: BTreeMap<usize, PageDirTab>,
    // A blob that the processor will read as a page directory.
    blob: Box<[u64; NUMBER_OF_ENTRIES]>,
}

/// A structure that represents the whole paging context.
#[derive(Debug)]
pub struct PagingContext {
    // A value that will loaded to CR3 when this paging context is used.
    cr3: u64,
    // A root page directory. This is PML4 in x86.
    directory: PageDirectory,
}

impl PagingContext {
    /// Find a physical address of the frame that is mapped by virtual address
    /// `virt_addr`.
    pub fn find(&self, virt_addr: usize) -> Option<usize> {
        let mut page_directory = &self.directory;
        let page_table;

        // Assert that the address is page aligned.
        assert_align(virt_addr);

        let indices = parse_addr(virt_addr);
        let mut i = 0;
        loop {
            // Traverse through the paging tree until we find the leave,
            // that is a page table.
            match page_directory.map.get(&indices[i])? {
                Directory(ref directory) => page_directory = directory,
                Table(ref table) => {
                    page_table = table;
                    break;
                }
            }
            i += 1;
        }
        i += 1;

        // Return the physical address mapped by virt_addr in the page table.
        Some(*page_table.map.get(&indices[i])?)
    }

    /// Map a page at virtual address `virt_addr` to a frame at physical
    /// address `phy_addr`.
    pub fn insert(&mut self, virt_addr: usize, phy_addr: usize)
        -> Result<(), ()>
    {
        // Assert that the virtual address is page aligned.
        assert_align(virt_addr);
        // Assert that the physical address is page aligned.
        assert_align(phy_addr);

        if self.find(virt_addr).is_some() {
            return Err(());
        }

        // Create two level page directory, that is PDPT and PD in x86.
        let indices = parse_addr(virt_addr);
        let mut i = 0;
        let mut page_directory = &mut self.directory;
        for _ in 0..2 {
            if page_directory.map.get(&indices[i]).is_none() {
                // If the page directory does not exist create a new one.
                let new_directory = Box::new(PageDirectory {
                    map: BTreeMap::new(),
                    blob: Box::new([0; NUMBER_OF_ENTRIES]),
                });
                // Insert the new directory to the map of the parent page
                // directory.
                page_directory.map.insert(
                    indices[i],
                    Directory(new_directory),
                );
            }

            // We need a tmp variable here to avoid Rust borrow checker.
            // https://goo.gl/BS92ft for more detail.
            let page_directory_ = page_directory;
            // Go deeper into the tree.
            match page_directory_.map.get_mut(&indices[i]).unwrap() {
                Directory(directory) => page_directory = &mut **directory,
                Table(_) => {
                    panic!("found page table, page directory expected");
                },
            }
            i += 1;
        }

        if page_directory.map.get(&indices[i]).is_none() {
            // If the page table does not exist create a new one.
            let new_table = Box::new(PageTable {
                map: BTreeMap::new(),
                blob: Box::new([0; NUMBER_OF_ENTRIES]),
            });
            // Insert the new table to the map of the parent page
            // directory.
            page_directory.map.insert(
                indices[i],
                Table(new_table),
            );
        }

        match page_directory.map.get_mut(&indices[i]).unwrap() {
            Directory(_) => {
                panic!("found page directory, page table expected");
            },
            Table(table) => {
                table.map.insert(indices[i+1], phy_addr);
            },
        }
        Ok(())
    }

    /// Create a new [PagingContext](PagingContext).
    pub fn new() -> PagingContext {
        let directory = PageDirectory {
            map: BTreeMap::new(),
            blob: Box::new([0; NUMBER_OF_ENTRIES]),
        };
        // We need to avoid setting .address in test because the address of
        // the blod in test is beyond the size of configured physical
        // address space.
        let cr3 = {
            #[cfg(not(test))]
            cr3! {
                .address = &*directory.blob as *const _ as u64
            }
            #[cfg(test)]
            0
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

#[cfg(test)]
use config::PAGE_SIZE;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_insert() {
        let mut context = PagingContext::new();
        let vir_addr1 = 4 * PAGE_SIZE;
        let phy_addr1 = 5 * PAGE_SIZE;
        let vir_addr2 = 2 * PAGE_SIZE;
        let phy_addr2 = 6 * PAGE_SIZE;
        assert!(context.find(vir_addr1).is_none());
        assert!(context.insert(vir_addr1, phy_addr1).is_ok());
        assert_eq!(context.find(vir_addr1).unwrap(), phy_addr1);

        assert!(context.find(vir_addr2).is_none());
        assert!(context.insert(vir_addr2, phy_addr2).is_ok());
        assert_eq!(context.find(vir_addr2).unwrap(), phy_addr2);
    }
}
