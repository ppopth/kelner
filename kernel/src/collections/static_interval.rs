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

use core::fmt;

/// The static size of the interval list.
#[cfg(not(test))]
const LIST_SIZE: usize = 0x100;
#[cfg(test)]
const LIST_SIZE: usize = 3;

/// The error type for [IntervalList](IntervalList).
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    MalformedInterval,
    ListFull,
}

/// An interval.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Interval {
    start: usize,
    length: usize,
}

impl Interval {
    /// Create [Interval](Interval) from `start` and `length`.
    pub fn new(start: usize, length: usize) -> Interval {
        Interval {
            start,
            length,
        }
    }

    /// Create [Interval](Interval) from a slice. For example, `0x1-0x3`.
    pub fn from(bytes: &[u8]) -> Result<Interval, ()> {
        let mut start = None;
        let mut end = None;

        let slice_to_number = |slice: &[u8]| -> Result<usize, ()> {
            let mut result: usize = 0;
            // To pase a slice, it must be hexadecimal and start with '0x'.
            if slice.len() < 3
                || slice.len() > 16 + 2
                || slice.split_at(2).0 != b"0x" {
                return Err(());
            }
            // Parse each digit in the number.
            for digit in slice.split_at(2).1.iter() {
                result <<= 4;
                match *digit {
                    b'a' ... b'f' => result += usize::from(*digit - b'a' + 10),
                    b'0' ... b'9' => result += usize::from(*digit - b'0'),
                    _ => return Err(()),
                }
            }
            Ok(result)
        };

        // We cannot use collect method because implementing the trait
        // FromIterator requires memory allocation.
        for address in bytes.split(|byte| *byte == b'-') {
            if start.is_none() {
                start = Some(slice_to_number(address)?);
            } else if end.is_none() {
                end = Some(slice_to_number(address)?);
            } else {
                return Err(());
            }
        }

        // Make sure that the interval contains exactly two numbers.
        if start.is_none() || end.is_none() {
            return Err(());
        }

        Ok(Interval::new(start.unwrap(), end.unwrap() - start.unwrap()))
    }
}

/// An iterator for [IntervalList](IntervalList).
pub struct Iter<'a> {
    index: usize,
    interval_list: &'a IntervalList,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Interval;
    /// Get the next item of the iterator.
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.interval_list.len() {
            None
        } else {
            let result = self.interval_list.list[self.index].as_ref().unwrap();
            self.index += 1;
            Some(result)
        }
    }
}

/// A static interval list which contains a list of intervals.
#[derive(Copy, Clone)]
pub struct IntervalList {
    list: [Option<Interval>; LIST_SIZE],
    len: usize,
}

impl IntervalList {
    /// Create an empty [IntervalList](IntervalList).
    pub fn new() -> IntervalList {
        IntervalList {
            list: [None; LIST_SIZE],
            len: 0,
        }
    }

    /// Return the length of the list.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Push an interval to the list.
    pub fn push(&mut self, interval: Interval) -> Result<(), Error> {
        // If it is going to go beyond the size, return error.
        if self.len >= LIST_SIZE {
            return Err(Error::ListFull);
        }
        self.list[self.len] = Some(interval);
        self.len += 1;
        Ok(())
    }

    /// Create [IntervalList](IntervalList) from a slice. For example,
    /// `0x1-0x3,0x3-0x5`.
    pub fn from(bytes: &[u8]) -> Result<IntervalList, Error> {
        let mut interval_list = IntervalList::new();

        // If the slice is empty, return empty IntervalList.
        if bytes.is_empty() {
            return Ok(interval_list);
        }

        for interval_bytes in bytes.split(|byte| *byte == b',') {
            let interval = match Interval::from(interval_bytes) {
                Ok(v) => v,
                Err(_) => return Err(Error::MalformedInterval),
            };

            interval_list.push(interval)?;
        }

        Ok(interval_list)
    }

    /// Get [Iter](Iter) of this interval list.
    pub fn iter(&self) -> Iter {
        Iter {
            index: 0,
            interval_list: self,
        }
    }

    /// Check if all intervals is covered by all intervals in the other list.
    pub fn is_covered_by(&self, other: &IntervalList) -> bool {
        let mut list1 = *self;
        let mut list2 = *other;
        list1.sort();
        list2.sort();

        let mut iter2 = list2.iter();
        let mut item2_opt = iter2.next();

        // This variable will determine the sweep line that indicates that
        // the range that comes before the line does not need to be covered,
        // since it is alrady covered or does not need to be covered.
        let mut line = 0;
        for item1 in list1.iter() {
            // If the sweep line is less than the start of the interval, we
            // know that the range line..item1.start doesn't need to be
            // covered. So we move the sweep line forward.
            if item1.start > line {
                line = item1.start;
            }

            // Items in list2 must try to eat the entire iterval item1.
            while let Some(item2) = item2_opt {
                // If the start of item2 comes before the sweep line, we
                // can process this item2 immediately. We don't have to wait
                // for the next item in list1.
                if item2.start <= line {
                    // If item2 covers beyond the line, we can move the line
                    // forward because the range line..item2.start+item2.length
                    // is already covered.
                    if item2.start + item2.length > line {
                        line = item2.start + item2.length;
                    }
                    item2_opt = iter2.next();
                } else {
                    break;
                }
            }

            // At the end of the loop, if the sweep line doesn't eat item1.
            // We can return false immediately.
            if line < item1.start + item1.length {
                return false;
            }
        }
        true
    }

    /// Sort all intervals in the list.
    pub fn sort(&mut self) {
        self.list[..self.len].sort_by(|a, b| a.unwrap().cmp(&b.unwrap()));
    }
}

impl fmt::Debug for IntervalList {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("IntervalList")
            .field("list", &&self.list[..self.len])
            .finish()
    }
}

impl PartialEq for IntervalList {
    fn eq(&self, other: &IntervalList) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for i in 0..self.len() {
            if !self.list[i].unwrap().eq(&other.list[i].unwrap()) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty_interval_list() {
        let interval_list = IntervalList::new();
        assert_eq!(
            format!("{:?}", interval_list),
            "IntervalList { list: [] }"
        );
        assert_eq!(interval_list.len(), 0);
    }

    #[test]
    fn parse_valid_empty_interval() {
        let interval_list = IntervalList::from(b"").unwrap();
        assert_eq!(
            format!("{:?}", interval_list),
            "IntervalList { list: [] }"
        );
        assert_eq!(interval_list.len(), 0);
    }

    #[test]
    fn parse_valid_interval() {
        let interval_list = IntervalList::from(b"0x1-0x2,0x3-0x4,0x5-0x6")
            .unwrap();
        assert_eq!(
            format!("{:x?}", interval_list),
            "IntervalList { list: [Some(Interval { start: 1, length: 1 }), \
Some(Interval { start: 3, length: 1 }), Some(Interval { start: 5, length: 1 \
})] }"
        );
        assert_eq!(interval_list.len(), 3);
    }

    #[test]
    fn parse_valid_large_interval() {
        let interval_list = IntervalList::from(b"\
0xfffffffffffffffe-0xffffffffffffffff,\
0xffffffffffffffff-0xffffffffffffffff\
").unwrap();
        assert_eq!(
            format!("{:x?}", interval_list),
            "IntervalList { list: [Some(Interval { start: fffffffffffffffe\
, length: 1 }), Some(Interval { start: ffffffffffffffff, length: 0 })] }"
        );
        assert_eq!(interval_list.len(), 2);
    }

    #[test]
    fn parse_invalid_interval() {
        assert_eq!(IntervalList::from(b"0xabk-0xfff").unwrap_err(),
                   Error::MalformedInterval);
    }

    #[test]
    fn parse_too_many_intervals() {
        assert_eq!(IntervalList::from(b"0x1-0x2,0x2-0x3,0x3-0x4,0x4-0x5")
                   .unwrap_err(), Error::ListFull);
    }

    #[test]
    fn push_valid_interval() {
        let mut interval_list = IntervalList::new();
        assert!(interval_list.push(Interval::new(1, 1)).is_ok());
        assert_eq!(interval_list.len(), 1);
    }

    #[test]
    fn push_when_full() {
        let mut interval_list = IntervalList::new();
        assert!(interval_list.push(Interval::new(1, 1)).is_ok());
        assert!(interval_list.push(Interval::new(2, 1)).is_ok());
        assert!(interval_list.push(Interval::new(3, 1)).is_ok());
        assert_eq!(
            interval_list.push(Interval::new(4, 1)).unwrap_err(),
            Error::ListFull
        )
    }

    #[test]
    fn eq_interval_list() {
        let mut list1 = IntervalList::new();
        let mut list2 = IntervalList::new();
        assert!(list1.push(Interval::new(1, 2)).is_ok());
        assert!(list1.push(Interval::new(2, 2)).is_ok());
        assert!(list2.push(Interval::new(1, 2)).is_ok());
        assert!(list2.push(Interval::new(2, 2)).is_ok());

        assert!(list1.eq(&list2));
    }

    #[test]
    fn shuffle_neq_interval_list() {
        let mut list1 = IntervalList::new();
        let mut list2 = IntervalList::new();
        assert!(list1.push(Interval::new(1, 2)).is_ok());
        assert!(list1.push(Interval::new(2, 2)).is_ok());
        assert!(list2.push(Interval::new(2, 2)).is_ok());
        assert!(list2.push(Interval::new(1, 2)).is_ok());

        assert!(!list1.eq(&list2));
    }

    #[test]
    fn size_neq_interval_list() {
        let mut list1 = IntervalList::new();
        let mut list2 = IntervalList::new();
        assert!(list1.push(Interval::new(1, 2)).is_ok());
        assert!(list1.push(Interval::new(2, 2)).is_ok());
        assert!(list2.push(Interval::new(1, 2)).is_ok());

        assert!(!list1.eq(&list2));
    }

    #[test]
    fn simple_cover() {
        let mut list1 = IntervalList::new();
        let mut list2 = IntervalList::new();
        assert!(list1.push(Interval::new(1, 2)).is_ok());
        assert!(list1.push(Interval::new(3, 2)).is_ok());
        assert!(list2.push(Interval::new(3, 2)).is_ok());
        assert!(list2.push(Interval::new(1, 2)).is_ok());

        assert!(list1.is_covered_by(&list2));
    }

    #[test]
    fn simple_not_cover() {
        let mut list1 = IntervalList::new();
        let mut list2 = IntervalList::new();
        assert!(list1.push(Interval::new(1, 2)).is_ok());
        assert!(list1.push(Interval::new(3, 3)).is_ok());
        assert!(list2.push(Interval::new(3, 2)).is_ok());
        assert!(list2.push(Interval::new(1, 2)).is_ok());

        assert!(!list1.is_covered_by(&list2));
    }

    #[test]
    fn interleave_cover() {
        let mut list1 = IntervalList::new();
        let mut list2 = IntervalList::new();
        assert!(list1.push(Interval::new(3, 4)).is_ok());
        assert!(list1.push(Interval::new(10, 4)).is_ok());
        assert!(list2.push(Interval::new(1, 4)).is_ok());
        assert!(list2.push(Interval::new(5, 7)).is_ok());
        assert!(list2.push(Interval::new(12, 5)).is_ok());

        assert!(list1.is_covered_by(&list2));
    }

    #[test]
    fn interleave_not_cover() {
        let mut list1 = IntervalList::new();
        let mut list2 = IntervalList::new();
        assert!(list1.push(Interval::new(3, 4)).is_ok());
        assert!(list1.push(Interval::new(10, 4)).is_ok());
        assert!(list2.push(Interval::new(5, 7)).is_ok());

        assert!(!list1.is_covered_by(&list2));
    }

    #[test]
    fn empty_coveree() {
        let list1 = IntervalList::new();
        let mut list2 = IntervalList::new();
        assert!(list2.push(Interval::new(5, 7)).is_ok());

        assert!(list1.is_covered_by(&list2));
    }

    #[test]
    fn empty_coverer() {
        let mut list1 = IntervalList::new();
        let list2 = IntervalList::new();
        assert!(list1.push(Interval::new(5, 7)).is_ok());

        assert!(!list1.is_covered_by(&list2));
    }
}
