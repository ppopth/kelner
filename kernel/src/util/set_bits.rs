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

use core::{ops, mem};

/// Set bits at position `start` to position `end`, not including `end`.
pub fn set_bits<T>(var: T, start: u8, end: u8, val: T) -> T
    where T: PartialOrd + From<u8>
             + ops::Shr<Output=T> + ops::Shl<Output=T>
             + ops::Not<Output=T> + ops::BitAnd<Output=T>
             + ops::AddAssign
{
    let size = (mem::size_of::<T>() * 8) as u8;
    assert!(end > start);
    assert!(end <= size);
    // Assert that val is less than 2^(end-start).
    assert!(val <= !(T::from(0)) >> T::from(size - end + start));

    // Reset bits from start to end, excluding end.
    let mut result = var & !((!(T::from(0)) >> T::from(size - end + start))
                             << T::from(start));
    // Put value into the valid position.
    result += val << T::from(start);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_input_u8() {
        let input: u8 = 0b11111111;
        assert_eq!(set_bits(input, 2, 5, 0b010), 0b11101011);
    }

    #[test]
    fn valid_input_u64() {
        let input: u64 = 0;
        assert_eq!(set_bits(input, 48, 64, 0xffff), 0xffff << 48);
    }

    #[test]
    #[should_panic]
    fn beyond_the_size_of_input() {
        set_bits(0 as u32, 0, 33, 0);
    }

    #[test]
    #[should_panic]
    fn too_large_value() {
        set_bits(0 as u32, 0, 2, 4);
    }
}
