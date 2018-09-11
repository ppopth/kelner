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
use siphasher::sip::SipHasher;

/// The static size of the hash table. The default is 0x100000.
#[cfg(not(test))]
const HASH_SIZE: usize = 0x100000;
#[cfg(test)]
const HASH_SIZE: usize = 3;

/// The error type for [StaticMap](StaticMap).
#[derive(Debug, Eq, PartialEq)]
enum Error {
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
#[derive(Copy, Clone)]
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
    where K: Hash + Copy + Eq, V: Copy
{
    /// Create an empty [StaticMap](self::StaticMap). All map entries are
    /// initially set to [None](None).
    fn new() -> StaticMap<K, V> {
        StaticMap {
            len: 0,
            ht: [None; HASH_SIZE],
        }
    }

    /// Return the number of elements in the map.
    fn len(&self) -> usize {
        self.len
    }

    /// Find a key in the map. If it can find, return the value of the entry.
    /// Otherwise, return [None](None).
    fn find(&self, key: K) -> Option<V> {
        // Valid position.
        let pos = key.hash() as usize % HASH_SIZE;
        let mut idx = pos;

        // Loop through entry hash table to find an entry.
        while (idx + 1) % HASH_SIZE != pos {
            // If we find the empty slot, this means that we cannot find the
            // input key at all.
            if let None = self.ht[idx] {
                return None;
            }
            if let Some(entry) = self.ht[idx] {
                if key == entry.key {
                    return Some(entry.value);
                }
            }
            // Go to the next hash entry.
            idx = (idx + 1) % HASH_SIZE;
        }

        return None;
    }

    /// Insert a key into the map with a value. If it can insert to the map,
    /// return the position in the hash table. Otherwise, return [None](None).
    fn insert(&mut self, key: K, value: V) -> Result<usize, Error> {
        // Valid position.
        let pos = key.hash() as usize % HASH_SIZE;
        let mut idx = pos;

        // Loop through entry hash table to find an empty slot.
        loop {
            if let Some(entry) = self.ht[idx] {
                // If the slot is not empty and is equal to the input key,
                // this means the key is already in the map. Return error.
                if entry.key == key {
                    return Err(Error::DuplicateKey);
                }
            } else {
                // If the slot is empty, add a new entry and increment the
                // length variable.
                self.ht[idx] = Some(StaticMapEntry {
                    key: key,
                    value: value,
                });
                self.len += 1;
                return Ok(idx);
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
