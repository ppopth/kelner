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

//! Kernel memory allocation module. This module uses slab allocation algorithm
//! to allocate kernel memory.

use core::alloc::{AllocErr, Layout};
use core::{ptr, mem};
use ::util::lg;
use ::collections::{
    StaticMap,
    StaticStack,
    StaticList,
    StaticListRef,
};
#[cfg(not(test))]
use ::config::{
    KERNEL_HEAP_START,
    KERNEL_HEAP_END,
};

/// The maximum slab size.
#[cfg(not(test))]
const SLAB_SIZE: usize = 0x1000;
// Log of SLAB_SIZE + 1.
#[cfg(not(test))]
const NUM_CACHES: usize = 12 + 1;

/// The value entry in the mapping of allocated addresses.
struct MapEntry {
    // The bit length of the allocation size. Maximum is NUM_CACHES-1.
    cache_index: usize,
    // The reference to an entry in the cache.
    cache_entry: StaticListRef<CacheEntry>,
}

/// This is a cache entry in the cache.
struct CacheEntry {
    // This is the physical address of the memory that actually stores user
    // data. Since this is an allocation module for the kernel memory and we
    // have an identity mapping between virtual addresses and physical
    // addresses, this address will always be the same as the virtual address.
    phy_addr: usize,
}

/// This is a cache for keeping free objects and allocated objects. There will
/// be one cache for one allocation size.
struct Cache {
    // Free entries in the cache.
    free_entries: StaticStack<CacheEntry>,
    // Allocated entries in the cache.
    allocated_entries: StaticList<CacheEntry>,
}

/// This structure will contains everything the kernel needs to know for
/// kernel memory allocation.
pub struct AllocContext {
    // This is a mapping between the address and the cache entry.
    addr_map: StaticMap<usize, MapEntry>,
    // List of caches.
    caches: [Cache; NUM_CACHES],
    // The next slab address that we can use. Note that this variable will
    // only increase because currently we assume that we can allocate slabs
    // but we cannot deallocate slab. If we want to deallocate slabs, we can
    // improve it later.
    next_slab_addr: usize,
}

/// Since currently we assume that there is only one core that can use this
/// allocation module, we can safely let this implement Sync.
unsafe impl Sync for AllocContext {}

impl AllocContext {
    /// Create an empty [AllocContext](AllocContext).
    pub fn new() -> AllocContext {
        let caches = unsafe {
            let mut array: [Cache; NUM_CACHES] = mem::uninitialized();
            for elem in &mut array {
                ptr::write(elem, Cache {
                    free_entries: StaticStack::new(),
                    allocated_entries: StaticList::new(),
                });
            }
            array
        };
        AllocContext {
            addr_map: StaticMap::new(),
            caches,
            next_slab_addr: KERNEL_HEAP_START,
        }
    }

    /// Allocate a kernel memory using a given layout.
    pub fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        // The size of memory that we want to allocate.
        let size = layout.align();
        let sizelg = lg(size).unwrap();
        // If the allocation size is too large, return error.
        if size > SLAB_SIZE {
            return Err(AllocErr);
        }

        let free_stack = &mut self.caches[sizelg].free_entries;
        let allocated_list = &mut self.caches[sizelg].allocated_entries;

        // If we are going beyond the heap section, return error.
        if free_stack.len() == 0
            && self.next_slab_addr >= KERNEL_HEAP_END {
            return Err(AllocErr);
        }

        // If the cache has no free entry, allocate a new slab and push
        // all new free entries into the stack.
        if free_stack.len() == 0 {

            // Add SLAB_SIZE/size entries to the free stack.
            for i in 0..SLAB_SIZE/size {
                // The address of ith entry of the slab.
                let phy_addr = i * size + self.next_slab_addr;

                free_stack.push(CacheEntry { phy_addr }).unwrap();
            }

            // Move next_slab_addr to the next available slab.
            self.next_slab_addr += SLAB_SIZE;
        }

        // Get a free entry from the stack.
        let entry = free_stack.pop().unwrap();
        let list_ref = allocated_list.push(entry).unwrap();

        let addr = allocated_list.get(&list_ref).phy_addr;
        // Add an entry to the map.
        self.addr_map.insert(addr, MapEntry {
            cache_index: sizelg,
            cache_entry: list_ref,
        }).unwrap();

        Ok(addr as *mut _)
    }

    /// Deallocate a kernel memory using a ptr and layout.
    pub fn dealloc(&mut self, ptr: *mut u8, _: Layout) -> Result<(), ()> {
        let addr = ptr as usize;
        let map_entry = self.addr_map.remove(addr)?;

        // Create new variables to make the code shorter.
        let cache_index = map_entry.cache_index;
        let cache_entry_ref = map_entry.cache_entry;

        let free_stack = &mut self.caches[cache_index].free_entries;
        let allocated_list = &mut self.caches[cache_index].allocated_entries;

        // Move the cache entry from the allocated list to the free stack.
        let cache_entry = allocated_list.remove(cache_entry_ref);
        free_stack.push(cache_entry).unwrap();

        Ok(())
    }
}

#[cfg(test)] const KERNEL_HEAP_START: usize = ::config::KERNEL_HEAP_START;
#[cfg(test)] const KERNEL_HEAP_END: usize = KERNEL_HEAP_START + 8;
#[cfg(test)] const SLAB_SIZE: usize = 4;
#[cfg(test)] const NUM_CACHES: usize = 2 + 1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocate_slab_size() {
        let mut context = AllocContext::new();
        let layout = Layout::from_size_align(4, 4).unwrap();

        assert!(context.alloc(layout).is_ok());
    }

    #[test]
    fn allocate_beyond_slab_size() {
        let mut context = AllocContext::new();
        let layout = Layout::from_size_align(8, 8).unwrap();

        assert_eq!(context.alloc(layout).unwrap_err(), AllocErr);
    }

    #[test]
    fn allocate_beyond_heap() {
        let mut context = AllocContext::new();
        let layout = Layout::from_size_align(2, 2).unwrap();

        assert!(context.alloc(layout).is_ok());
        assert!(context.alloc(layout).is_ok());
        assert!(context.alloc(layout).is_ok());
        assert!(context.alloc(layout).is_ok());
        assert_eq!(context.alloc(layout).unwrap_err(), AllocErr);
    }

    #[test]
    fn allocate_slab_size_beyond_heap() {
        let mut context = AllocContext::new();
        let layout = Layout::from_size_align(4, 4).unwrap();

        assert!(context.alloc(layout).is_ok());
        assert!(context.alloc(layout).is_ok());
        assert_eq!(context.alloc(layout).unwrap_err(), AllocErr);
    }

    #[test]
    fn allocate_different_sizes() {
        let mut context = AllocContext::new();
        let layout1 = Layout::from_size_align(2, 2).unwrap();
        let layout2 = Layout::from_size_align(2, 4).unwrap();
        let layout3 = Layout::from_size_align(1, 1).unwrap();

        assert!(context.alloc(layout1).is_ok());
        assert!(context.alloc(layout2).is_ok());
        // The last one should fail because it will need three slabs for
        // three different sizes.
        assert_eq!(context.alloc(layout3).unwrap_err(), AllocErr);
    }

    #[test]
    fn different_valid_allocated_addresses() {
        let mut context = AllocContext::new();
        let layout = Layout::from_size_align(2, 2).unwrap();
        let mut output: [*mut u8; 4] = [ptr::null_mut(); 4];
        let mut expected: [*mut u8; 4] = [ptr::null_mut(); 4];

        for i in 0..4 {
            output[i] = context.alloc(layout).unwrap();
            expected[i] = (KERNEL_HEAP_START + i * 2) as *mut _;
        }

        output.sort();
        expected.sort();
        assert_eq!(output, expected);
    }

    #[test]
    #[should_panic]
    fn dealloc_non_allocated_address() {
        let mut context = AllocContext::new();
        let layout = Layout::from_size_align(1, 1).unwrap();

        // This should return error.
        context.dealloc(KERNEL_HEAP_START as *mut _, layout).unwrap();
    }

    #[test]
    fn dealloc_alternate_with_alloc() {
        let mut context = AllocContext::new();
        let layout = Layout::from_size_align(2, 2).unwrap();

        let a1 = context.alloc(layout).unwrap();
        let a2 = context.alloc(layout).unwrap();
        let a3 = context.alloc(layout).unwrap();
        let a4 = context.alloc(layout).unwrap();
        assert!(context.dealloc(a1, layout).is_ok());
        assert!(context.dealloc(a3, layout).is_ok());
        let a5 = context.alloc(layout).unwrap();
        let a6 = context.alloc(layout).unwrap();
        assert!(context.dealloc(a5, layout).is_ok());
        assert!(context.dealloc(a6, layout).is_ok());
        assert!(context.dealloc(a2, layout).is_ok());
        assert!(context.dealloc(a4, layout).is_ok());
    }
}
