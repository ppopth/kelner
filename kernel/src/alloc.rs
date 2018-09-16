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

use core::alloc::{GlobalAlloc, Layout};
use core::num::{NonZeroU64, NonZeroUsize};
use core::{ptr, mem};
use ::config;
use ::util::{StaticMap, StaticList, StaticListRef};

/// The maximum slab size.
const SLAB_SIZE: usize = config::PAGE_SIZE;
const SLAB_SIZE_LOG: usize = config::PAGE_SIZE_LOG;

static mut CONTEXT: Option<AllocationContext> = None;

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
    phy_addr: NonZeroU64,
}

/// This is a cache for keeping free objects and allocated objects. There will
/// be one cache for one allocation size.
struct Cache {
    // Free entries in the cache.
    free_entries: StaticList<CacheEntry>,
    // Allocated entries in the cache.
    allocated_entries: StaticList<CacheEntry>,
}

impl Cache {
    /// Create an empty [Cache](Cache).
    pub fn new() -> Cache {
        Cache {
            free_entries: StaticList::new(),
            allocated_entries: StaticList::new(),
        }
    }
}

/// This structure will contains everything the kernel needs to know for
/// kernel memory allocation.
pub struct AllocationContext {
    // This is a mapping between the address and the cache entry.
    addr_map: StaticMap<NonZeroU64, MapEntry>,
    // List of caches.
    caches: [Cache; SLAB_SIZE_LOG],
    // The next slap address that we can use. Note that this variable will
    // only increase because currently we assume that we can allocate slaps
    // but we cannot deallocate slap. If we want to deallocate slaps, we can
    // improve it later.
    next_slap_addr: u64,
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
                ptr::write(elem, Cache::new());
            }
            array
        };
        AllocationContext {
            addr_map: StaticMap::new(),
            caches,
            next_slap_addr: config::KERNEL_HEAP_START,
        }
    }
}

/// Empty structure to used in Rust's `global_allocator` feature.
pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 { ptr::null_mut() }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

/// Initialization function for the entire kernel memory allocation module.
pub fn init() {
    unsafe {
        CONTEXT = Some(AllocationContext::new());
    }
}
