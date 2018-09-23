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
#[derive(Copy, Clone, Debug)]
struct Interval {
    start: usize,
    end: usize,
}

impl Interval {
    /// Create [Interval](Interval) from `start` and `end`.
    pub fn new(start: usize, end: usize) -> Result<Interval, ()> {
        // Make sure that the start is not greater than end.
        if start > end {
            return Err(());
        }

        Ok(Interval {
            start,
            end,
        })
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

        Interval::new(start.unwrap(), end.unwrap())
    }
}

/// A static interval list which contains a list of intervals.
struct IntervalList {
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
}

impl fmt::Debug for IntervalList {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("IntervalList")
            .field("list", &&self.list[..self.len])
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn new_invalid_interval() {
        Interval::new(2,1).unwrap();
    }

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
            "IntervalList { list: [Some(Interval { start: 1, end: 2 }), \
Some(Interval { start: 3, end: 4 }), Some(Interval { start: 5, end: 6 })] }"
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
, end: ffffffffffffffff }), Some(Interval { start: ffffffffffffffff, end: \
ffffffffffffffff })] }"
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
        assert!(interval_list.push(Interval::new(1, 2).unwrap()).is_ok());
        assert_eq!(interval_list.len(), 1);
    }

    #[test]
    fn push_when_full() {
        let mut interval_list = IntervalList::new();
        assert!(interval_list.push(Interval::new(1, 2).unwrap()).is_ok());
        assert!(interval_list.push(Interval::new(2, 3).unwrap()).is_ok());
        assert!(interval_list.push(Interval::new(3, 4).unwrap()).is_ok());
        assert_eq!(
            interval_list.push(Interval::new(4, 5).unwrap()).unwrap_err(),
            Error::ListFull
        )
    }
}
