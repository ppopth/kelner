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

use core::ptr::NonNull;
use core::{mem, ptr};
use ::util::WeakRng;
use ::util::Usize;

/// The static size of the linked list.
#[cfg(not(test))]
const LIST_SIZE: usize = 0x0000_1000;
#[cfg(test)]
const LIST_SIZE: usize = 6;

/// The error type for [StaticList](StaticList).
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    ListFull,
}

/// An entry in [StaticList](StaticList).
struct StaticListElement<T> {
    // The item that this element contains.
    item: T,
    // The link to the previous element.
    prev: Option<Usize>,
    // The link to the next element.
    next: Option<Usize>,
}

/// A reference used by external users to reference items in the list.
// You can see that the index here is Usize, because, if so, the size of
// StaticListElement can be easily optimized.
#[derive(Debug, Eq, PartialEq)]
pub struct StaticListRef<T> {
    // The index to which this reference is referring to.
    index: Usize,
    // The address of the list to which this reference belongs to. At the first
    // glance, you may think that this reference may refer to two different
    // lists, if you allocate a new list located at the same address of the
    // freed list. In fact, that can't happen, if you allocate the list as
    // a static list, i.e. it isn't allocated in the heap or the stack.
    list: NonNull<StaticList<T>>,
}

/// The structure that is used when you don't want to allocate memory in
/// the heap. This can be done because it uses the data and bss
/// sections instead.
pub struct StaticList<T> {
    // The buffer to store all slots.
    buf: [Option<StaticListElement<T>>; LIST_SIZE],
    // The number generator for finding an empty slot.
    wrng: WeakRng,
    // The head index of the list.
    head: Option<Usize>,
    // The length of the list.
    len: usize,
}

impl<T> StaticList<T>
{
    /// Make sure that the ref is valid for this list.
    fn assert_ref(&self, refer: &StaticListRef<T>) {
        if refer.list.as_ptr() as usize != self as *const _ as usize {
            panic!("invalid StaticListRef<T> instance");
        }
    }

    /// Find an empty slot in the list buffer.
    fn find_empty_slot(&mut self) -> Result<usize, Error>
    {
        // If the list is already full, we cannot add more element to it.
        if self.len == LIST_SIZE {
            return Err(Error::ListFull);
        }
        // If the list is not full, find an empty one.
        loop {
            let random_index = self.wrng.next() as usize % LIST_SIZE;
            if self.buf[random_index].is_none() {
                return Ok(random_index);
            }
        }
    }

    /// Create an empty [StaticList](self::StaticList).
    pub fn new() -> StaticList<T> {
        let buf = unsafe {
            let mut array: [Option<StaticListElement<T>>; LIST_SIZE] =
                                                        mem::uninitialized();
            for elem in array.iter_mut() {
                ptr::write(elem, None);
            }
            array
        };
        StaticList {
            buf,
            wrng: WeakRng::new(),
            head: None,
            len: 0,
        }
    }

    /// Return the length of the list.
    pub fn len(&self) -> usize {
       self.len
    }

    /// Remove some specific element from the list.
    pub fn remove(&mut self, refer: StaticListRef<T>) -> T {
        self.assert_ref(&refer);

        let (elem_prev, elem_next) = {
            let element = self.buf[refer.index.get()].as_ref().unwrap();
            if let Some(v) = self.head {
                // This means that we are trying to remove the head of
                // the list.
                if v == refer.index {
                    self.head = element.next;
                }
            }
            (element.prev, element.next)
        };

        // Fix the linking chain.
        if let Some(prev) = elem_prev {
            self.buf[prev.get()].as_mut().unwrap().next = elem_next;
        }
        if let Some(next) = elem_next {
            self.buf[next.get()].as_mut().unwrap().prev = elem_prev;
        }

        // Decrease the length of the list.
        self.len -= 1;
        let index = refer.index.get();
        mem::drop(refer);

        // Remove the element from the list and return the item inside.
        self.buf[index].take().unwrap().item
    }

    /// Get an item using a reference.
    pub fn get(&self, refer: &StaticListRef<T>) -> &T {
        self.assert_ref(refer);

        &self.buf[refer.index.get()].as_ref().unwrap().item
    }

    /// Pust a new element in front of the list.
    pub fn push(&mut self, item: T) -> Result<StaticListRef<T>, Error>
    {
        // Find a new empty slot in the list.
        let new_slot_index = Usize::new(self.find_empty_slot()?);

        // If the list is not full, we can create a new list element
        // containing a parameter item.
        let mut new_element = StaticListElement {
            item,
            prev: None,
            next: None,
        };

        // Link a new element to the list.
        self.head = match self.head {
            Some(v) => {
                // Link a new element to an old element.
                new_element.next = Some(v);
                // Link an old element to a new element.
                let head_item = self.buf[v.get()].as_mut().unwrap();
                head_item.prev = Some(new_slot_index);

                Some(new_slot_index)
            },
            None => Some(new_slot_index),
        };

        // Increase the length.
        self.len += 1;

        self.buf[new_slot_index.get()] = Some(new_element);

        Ok(StaticListRef {
            index: new_slot_index,
            list: NonNull::new(self).unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_correct_item() {
        let mut list: StaticList<u64> = StaticList::new();
        let e1 = list.push(10).unwrap();
        let e2 = list.push(20).unwrap();
        let e3 = list.push(30).unwrap();
        assert_eq!(*list.get(&e3), 30);
        assert_eq!(*list.get(&e2), 20);
        assert_eq!(*list.get(&e1), 10);
        assert_eq!(list.remove(e3), 30);
        assert_eq!(*list.get(&e2), 20);
        assert_eq!(*list.get(&e1), 10);
    }

    #[test]
    fn return_correct_len() {
        let mut list: StaticList<u64> = StaticList::new();

        assert_eq!(list.len(), 0);

        let e1 = list.push(1).unwrap();
        assert_eq!(list.len(), 1);

        let e2 = list.push(2).unwrap();
        assert_eq!(list.len(), 2);

        let e3 = list.push(3).unwrap();
        assert_eq!(list.len(), 3);

        assert_eq!(list.remove(e2), 2);
        assert_eq!(list.len(), 2);

        assert_eq!(list.remove(e1), 1);
        assert_eq!(list.len(), 1);

        assert_eq!(list.remove(e3), 3);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn push_alternate_with_remove() {
        let mut list: StaticList<u64> = StaticList::new();

        assert_eq!(list.len(), 0);

        let e1 = list.push(1).unwrap();
        let e2 = list.push(2).unwrap();
        assert_eq!(list.remove(e2), 2);
        let e3 = list.push(3).unwrap();
        assert_eq!(list.remove(e1), 1);
        assert_eq!(list.remove(e3), 3);
    }

    #[test]
    fn push_when_full() {
        let mut list: StaticList<u64> = StaticList::new();
        assert!(list.push(1).is_ok());
        assert!(list.push(2).is_ok());
        assert!(list.push(3).is_ok());
        assert!(list.push(4).is_ok());
        assert!(list.push(5).is_ok());
        assert!(list.push(6).is_ok());

        // This one should return error, because the map is full already.
        assert_eq!(list.push(7).unwrap_err(), Error::ListFull);
    }

    #[test]
    #[should_panic]
    fn use_reference_from_another_list() {
        let mut list1: StaticList<u64> = StaticList::new();
        let mut list2: StaticList<u64> = StaticList::new();

        let e = list1.push(1).unwrap();
        list2.remove(e);
    }
}
