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

use core::{mem, ptr};

/// The static size of the stack.
#[cfg(not(test))]
const STACK_SIZE: usize = 0x0001_0000;
#[cfg(test)]
const STACK_SIZE: usize = 6;

/// An entry in [StaticStack](StaticStack).
pub struct StaticStack<T> {
    // The buffer to store all items in the stack.
    buf: [Option<T>; STACK_SIZE],
    // The number of items in the stack.
    len: usize,
}

impl<T> StaticStack<T> {
    /// Create an empty [StaticStack](self::StaticStack).
    pub fn new() -> StaticStack<T> {
        let buf = unsafe {
            let mut array: [Option<T>; STACK_SIZE] = mem::uninitialized();
            for elem in array.iter_mut() {
                ptr::write(elem, None);
            }
            array
        };
        StaticStack {
            buf,
            len: 0,
        }
    }

    /// Return the number of items in the stack.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Pop an item from the stack.
    pub fn pop(&mut self) -> Result<T, ()> {
        if self.len == 0 {
            return Err(());
        }
        let item = self.buf[self.len-1].take().unwrap();
        self.len -= 1;
        Ok(item)
    }

    /// Push a new item into the stack.
    pub fn push(&mut self, item: T) -> Result<&T, ()> {
        // If the stack is already full, return error.
        if self.len == STACK_SIZE {
            return Err(());
        }

        // Add a new item to the stack.
        self.buf[self.len] = Some(item);
        // Increase the size of the stack.
        self.len += 1;
        Ok(self.buf[self.len-1].as_ref().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_correct_item() {
        let mut stack: StaticStack<u64> = StaticStack::new();
        assert!(stack.push(10).is_ok());
        assert!(stack.push(20).is_ok());
        assert!(stack.push(30).is_ok());
        assert_eq!(stack.pop().unwrap(), 30);
        assert_eq!(stack.pop().unwrap(), 20);
        assert_eq!(stack.pop().unwrap(), 10);
    }

    #[test]
    fn return_correct_len() {
        let mut stack: StaticStack<u64> = StaticStack::new();

        assert_eq!(stack.len(), 0);

        assert!(stack.push(1).is_ok());
        assert_eq!(stack.len(), 1);

        assert!(stack.push(2).is_ok());
        assert_eq!(stack.len(), 2);

        assert!(stack.push(3).is_ok());
        assert_eq!(stack.len(), 3);

        assert!(stack.pop().is_ok());
        assert_eq!(stack.len(), 2);

        assert!(stack.pop().is_ok());
        assert_eq!(stack.len(), 1);

        assert!(stack.pop().is_ok());
        assert_eq!(stack.len(), 0);
    }

    #[test]
    #[should_panic]
    fn pop_when_empty() {
        let mut stack: StaticStack<u64> = StaticStack::new();

        // This one should return error, because the stack is empty.
        stack.pop().unwrap();
    }

    #[test]
    #[should_panic]
    fn push_when_full() {
        let mut stack: StaticStack<u64> = StaticStack::new();
        assert!(stack.push(1).is_ok());
        assert!(stack.push(2).is_ok());
        assert!(stack.push(3).is_ok());
        assert!(stack.push(4).is_ok());
        assert!(stack.push(5).is_ok());
        assert!(stack.push(6).is_ok());

        // This one should return error, because the stack is full already.
        stack.push(6).unwrap();
    }
}
