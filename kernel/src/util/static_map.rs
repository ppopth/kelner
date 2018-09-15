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
#![cfg_attr(not(test), allow(dead_code))]

use core::hash::Hasher;
use core::{mem, ptr};
use siphasher::sip::SipHasher;

/// The static size of the hash table. The default is 0x100000.
#[cfg(not(test))]
const HASH_SIZE: usize = 0x0010_0000;
#[cfg(test)]
const HASH_SIZE: usize = 3;

/// The error type for [StaticMap](StaticMap).
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    NotFound,
    DuplicateKey,
    MapFull,
}

/// A trait which ensures that the implementor can be hashed to [u64](u64).
pub trait Hash {
    fn hash(&self) -> u64;
}

impl Hash for u64 {
    fn hash(&self) -> u64 {
        let mut hasher = SipHasher::new();
        hasher.write_u64(*self);
        hasher.finish()
    }
}

/// An entry in [StaticMap](StaticMap).
struct StaticMapEntry<K, V> {
    key: K,
    value: V,
}

/// The structure that is used when you don't want to allocate memory in
/// the heap. This can be done because it uses the data and bss
/// sections instead.
pub struct StaticMap<K, V>
{
    // The number of elements in the map.
    len: usize,
    // The hash table to store elements in the map.
    ht: [Option<StaticMapEntry<K, V>>; HASH_SIZE],
}

impl<K, V> StaticMap<K, V>
    where K: Hash + Copy + Eq
{
    /// Create an empty [StaticMap](self::StaticMap). All map entries are
    /// initially set to [None](None).
    pub fn new() -> StaticMap<K, V> {
        let ht = unsafe {
            let mut array: [Option<StaticMapEntry<K, V>>; HASH_SIZE] =
                                                        mem::uninitialized();
            for elem in array.iter_mut() {
                ptr::write(elem, None);
            }
            array
        };
        StaticMap {
            len: 0,
            ht,
        }
    }

    /// Return the number of elements in the map.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Find a key in the map. If it can find, return the value of the entry.
    /// Otherwise, return [None](None).
    pub fn find(&self, key: K) -> Option<&V> {
        self.find_idx(key).map(|idx| &self.ht[idx].as_ref().unwrap().value)
    }

    /// Find a key in the map. If it can find, return the position in the
    /// hash table. Otherwise, return [None](None).
    pub fn find_idx(&self, key: K) -> Option<usize> {
        // Valid position.
        let pos = key.hash() as usize % HASH_SIZE;
        let mut idx = pos;

        // Loop through entry hash table to find an entry.
        while (idx + 1) % HASH_SIZE != pos {
            // If we find the empty slot, this means that we cannot find the
            // input key at all.
            self.ht[idx].as_ref()?;
            if let Some(entry) = self.ht[idx].as_ref() {
                if key == entry.key {
                    return Some(idx);
                }
            }
            // Go to the next hash entry.
            idx = (idx + 1) % HASH_SIZE;
        }

        None
    }

    /// Remove an entry from the map using a key.
    pub fn remove(&mut self, key: K) -> Result<V, Error> {
        let idx = match self.find_idx(key) {
            Some(v) => v,
            None => return Err(Error::NotFound),
        };
        self.remove_idx(idx)
    }

    /// Remove an entry from the map using an index in the hash table.
    pub fn remove_idx(&mut self, idx: usize) -> Result<V, Error> {
        // Keep the value.
        let value = match self.ht[idx].take() {
            Some(e) => e.value,
            None => return Err(Error::NotFound),
        };

        // The algorithm here will iterate through each entry in the hash
        // table until it finds either an empty entry or an entry with hash
        // value not in a range it has traversed. If it's the former case,
        // it will just terminate. If it's the latter case, it will move the
        // entry from that position `pos` to the position at `idx` and it wil
        // treat it as if the entry at `pos` is removed.
        let mut removed_idx = Some(idx);
        loop {
            // If it doesn't remove any item in the previous iteration,
            // the hash table is already in the correct state.
            if removed_idx.is_none() {
                break;
            }
            // Remove from the hash table.
            self.ht[removed_idx.unwrap()] = None;
            let mut i = (removed_idx.unwrap() + 1) % HASH_SIZE;

            loop {
                let pos = match self.ht[i] {
                    Some(ref entry) => entry.key.hash() as usize % HASH_SIZE,
                    None => {
                        removed_idx = None;
                        break;
                    }
                };
                if (i > idx && (pos <= idx || pos > i)) ||
                   (i < idx && (pos > i && pos <= idx)) {
                    self.ht[idx] = self.ht[i].take();
                    removed_idx = Some(i);
                    break;
                }

                i = (i + 1) % HASH_SIZE;
            }
        }
        Ok(value)
    }

    /// Insert a key into the map with a value. If it can insert to the map,
    /// return the position in the hash table. Otherwise, return [None](None).
    pub fn insert(&mut self, key: K, value: V) -> Result<usize, Error> {
        // Valid position.
        let pos = key.hash() as usize % HASH_SIZE;
        let mut idx = pos;

        // Loop through entry hash table to find an empty slot.
        loop {
            if self.ht[idx].is_none() {
                // If the slot is empty, add a new entry and increment the
                // length variable.
                self.ht[idx] = Some(StaticMapEntry {
                    key,
                    value,
                });
                self.len += 1;
                return Ok(idx);
            }

            // If the slot is not empty and is equal to the input key,
            // this means the key is already in the map. Return error.
            if self.ht[idx].as_ref().unwrap().key == key {
                return Err(Error::DuplicateKey);
            }

            idx = (idx + 1) % HASH_SIZE;
            if idx == pos {
                break;
            }
        }
        Err(Error::MapFull)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_return_correct_value() {
        let mut map: StaticMap<u64, u64> = StaticMap::new();
        assert!(map.insert(10, 9).is_ok());
        assert!(map.insert(20, 99).is_ok());
        assert!(map.insert(30, 999).is_ok());

        assert_eq!(*map.find(10).unwrap(), 9);
        assert_eq!(*map.find(20).unwrap(), 99);
        assert_eq!(*map.find(30).unwrap(), 999);
    }

    #[test]
    fn remove_non_existing_entry() {
        let mut map: StaticMap<u64, u64> = StaticMap::new();
        assert!(map.insert(0, 0).is_ok());
        assert!(map.insert(1, 0).is_ok());

        // This one should return error, because the entry doesn't exist.
        assert_eq!(map.remove(2).unwrap_err(), Error::NotFound);
    }

    #[test]
    fn remove_entries_correctly() {
        let mut map: StaticMap<u64, u64> = StaticMap::new();
        assert!(map.insert(0, 10).is_ok());
        assert!(map.insert(1, 20).is_ok());
        assert!(map.insert(2, 30).is_ok());
        assert_eq!(map.remove(1).unwrap(), 20);
        assert!(map.insert(3, 40).is_ok());
        assert_eq!(map.remove(2).unwrap(), 30);
        assert_eq!(map.remove(3).unwrap(), 40);
        assert_eq!(map.remove(0).unwrap(), 10);
    }

    #[test]
    fn insert_when_full() {
        let mut map: StaticMap<u64, u64> = StaticMap::new();
        assert!(map.insert(0, 0).is_ok());
        assert!(map.insert(1, 0).is_ok());
        assert!(map.insert(2, 0).is_ok());

        // This one should return error, because the map is full already.
        assert_eq!(map.insert(3, 0).unwrap_err(), Error::MapFull);
    }

    #[test]
    fn insert_duplicate_key() {
        let mut map: StaticMap<u64, u64> = StaticMap::new();
        assert!(map.insert(0, 0).is_ok());
        assert!(map.insert(1, 0).is_ok());

        // This one should return error, because the key is duplicate.
        assert_eq!(map.insert(1, 0).unwrap_err(), Error::DuplicateKey);
        // And the len also remains untact.
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn return_correct_len() {
        let mut map: StaticMap<u64, u64> = StaticMap::new();
        assert!(map.insert(0, 0).is_ok());
        assert!(map.insert(1, 0).is_ok());

        assert_eq!(map.len(), 2);
    }

    #[test]
    fn find_non_existing_key() {
        let mut map: StaticMap<u64, u64> = StaticMap::new();
        assert!(map.insert(0, 0).is_ok());

        // This one should return None, because the key isn't in the map.
        assert_eq!(map.find(1), None);
    }
}
