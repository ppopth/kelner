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

        // Make sure that the start is not greater than end.
        if start.unwrap() > end.unwrap() {
            return Err(());
        }

        Ok(Interval {
            start: start.unwrap(),
            end: end.unwrap(),
        })
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

    /// Create [IntervalList](IntervalList) from a slice. For example,
    /// `0x1-0x3,0x3-0x5`.
    pub fn from(bytes: &[u8]) -> Result<IntervalList, Error> {
        let mut list = [None; LIST_SIZE];
        let mut len = 0;

        // If the slice is empty, return empty IntervalList.
        if bytes.is_empty() {
            return Ok(IntervalList {
                list,
                len,
            });
        }

        for interval_bytes in bytes.split(|byte| *byte == b',') {
            let interval = match Interval::from(interval_bytes) {
                Ok(v) => v,
                Err(_) => return Err(Error::MalformedInterval),
            };
            // If it is going to go beyond the size, return error.
            if len >= LIST_SIZE {
                return Err(Error::ListFull);
            }
            list[len] = Some(interval);
            len += 1;
        }

        Ok(IntervalList {
            list,
            len,
        })
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
    fn new_empty_interval_list() {
        assert_eq!(
            format!("{:?}", IntervalList::new()),
            "IntervalList { list: [] }"
        );
    }

    #[test]
    fn parse_valid_empty_interval() {
        assert_eq!(
            format!("{:?}", IntervalList::from(b"")),
            "Ok(IntervalList { list: [] })"
        );
    }

    #[test]
    fn parse_valid_interval() {
        assert_eq!(
            format!("{:x?}", IntervalList::from(b"0x1-0x2,0x3-0x4,0x5-0x6")),
            "Ok(IntervalList { list: [Some(Interval { start: 1, end: 2 }), \
Some(Interval { start: 3, end: 4 }), Some(Interval { start: 5, end: 6 })] })"
        );
    }

    #[test]
    fn parse_valid_large_interval() {
        assert_eq!(
            format!("{:x?}", IntervalList::from(b"\
0xfffffffffffffffe-0xffffffffffffffff,\
0xffffffffffffffff-0xffffffffffffffff\
")),
            "Ok(IntervalList { list: [Some(Interval { start: fffffffffffffffe\
, end: ffffffffffffffff }), Some(Interval { start: ffffffffffffffff, end: \
ffffffffffffffff })] })"
        );
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
}
