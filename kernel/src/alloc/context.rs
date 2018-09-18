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

//! Kernel memory allocation module. This module uses slab allocation algorithm
//! to allocate kernel memory.

use core::alloc::{AllocErr, Layout};
use core::num::NonZeroUsize;
use core::{ptr, mem};
use ::collections::{
    StaticMap,
    StaticStack,
    StaticList,
    StaticListRef,
};
use ::config::{
    KERNEL_HEAP_START,
    KERNEL_HEAP_END,
    PAGE_SIZE,
    PAGE_SIZE_LOG,
};

/// The maximum slab size.
const SLAB_SIZE: usize = PAGE_SIZE;
const SLAB_SIZE_LOG: usize = PAGE_SIZE_LOG;

/// The value entry in the mapping of allocated addresses.
struct MapEntry {
    // The bit length of the allocation size. Maximum is SLAB_SIZE_LOG.
    len: NonZeroUsize,
    // The reference to an entry in the cache.
    cache_entry: StaticListRef<CacheEntry>,
}

/// This is a cache entry in the cache.
struct CacheEntry {
    // This is the physical address of the memory that actually stores user
    // data. Since this is an allocation module for the kernel memory and we
    // have an identity mapping between virtual addresses and physical
    // addresses, this address will always be the same as the virtual address.
    phy_addr: NonZeroUsize,
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
pub struct AllocationContext {
    // This is a mapping between the address and the cache entry.
    addr_map: StaticMap<NonZeroUsize, MapEntry>,
    // List of caches.
    caches: [Cache; SLAB_SIZE_LOG],
    // The next slab address that we can use. Note that this variable will
    // only increase because currently we assume that we can allocate slabs
    // but we cannot deallocate slab. If we want to deallocate slabs, we can
    // improve it later.
    next_slab_addr: NonZeroUsize,
}

/// Since currently we assume that there is only one core that can use this
/// allocation module, we can safely let this implement Sync.
unsafe impl Sync for AllocationContext {}

impl AllocationContext {
    /// Create an empty [AllocationContext](AllocationContext).
    pub fn new() -> AllocationContext {
        let caches = unsafe {
            let mut array: [Cache; SLAB_SIZE_LOG] = mem::uninitialized();
            for elem in &mut array {
                ptr::write(elem, Cache {
                    free_entries: StaticStack::new(),
                    allocated_entries: StaticList::new(),
                });
            }
            array
        };
        AllocationContext {
            addr_map: StaticMap::new(),
            caches,
            next_slab_addr: NonZeroUsize::new(KERNEL_HEAP_START).unwrap(),
        }
    }

    /// Allocate a kernel memory using a given layout.
    pub fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        let align = layout.align();
        // If the allocation size is too large, return error.
        if align > SLAB_SIZE {
            return Err(AllocErr);
        }

        // TODO:
        let free_stack = &mut self.caches[align].free_entries;
        // TODO:
        let allocated_list = &mut self.caches[align].allocated_entries;
        let nonzero_align = NonZeroUsize::new(align).unwrap();

        // If we are going beyond the heap section, return error.
        if free_stack.len() == 0
            && self.next_slab_addr.get() >= KERNEL_HEAP_END {
            return Err(AllocErr);
        }

        // If the cache has no free entry, allocate a new slab and push
        // all new free entries into the stack.
        if free_stack.len() == 0 {

            // Add SLAB_SIZE/align entries to the free stack.
            for i in 0..SLAB_SIZE/align {
                // The address of ith entry of the slab.
                let phy_addr = NonZeroUsize::new(
                    i * align + self.next_slab_addr.get()
                ).unwrap();

                free_stack.push(CacheEntry { phy_addr }).unwrap();
            }

            self.next_slab_addr = NonZeroUsize::new(
                self.next_slab_addr.get() + SLAB_SIZE
            ).unwrap();
        }

        // Get a free entry from the stack.
        let entry = free_stack.pop().unwrap();
        let list_ref = allocated_list.push(entry).unwrap();

        let addr = allocated_list.get(&list_ref).phy_addr;
        // Add an entry to the map.
        self.addr_map.insert(addr, MapEntry {
            // TODO:
            len: nonzero_align,
            cache_entry: list_ref,
        }).unwrap();

        Ok(addr.get() as *mut _)
    }

    /// Deallocate a kernel memory using a ptr and layout.
    pub fn dealloc(&mut self, ptr: *mut u8, _layout: Layout) {
        let addr = NonZeroUsize::new(ptr as usize).unwrap();
        let map_entry = match self.addr_map.remove(addr) {
            Ok(v) => v,
            Err(_) => return,
        };

        let _len = map_entry.len;
        let _cach_entry = map_entry.cache_entry;
    }
}
