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

/// Set bits at position `start` to position `end`, not including `end`.
/// # Example
/// ```
/// let input: u8 = 0b10101100;
/// let result = util::set_bits(input, 2, 5, 0b100);
/// assert_eq!(result, 0b10110000);
/// ```
pub fn set_bits<T>(var: T, start: u8, end: u8, val: T) -> T
    where T: PartialOrd + From<u8>
             + std::ops::Shr<Output=T> + std::ops::Shl<Output=T>
             + std::ops::Not<Output=T> + std::ops::BitAnd<Output=T>
             + std::ops::AddAssign
{
    let size = (std::mem::size_of::<T>() * 8) as u8;
    assert!(end > start);
    assert!(end <= size);
    // Assert that val is less than 2^(end-start).
    assert!(val < !(T::from(0)) >> T::from(size - end + start));

    // Reset bits from start to end, excluding end.
    let mut result = var & !((!(T::from(0)) >> T::from(size - end + start))
                             << T::from(start));
    // Put value into the valid position.
    result += val << T::from(start);
    result
}
