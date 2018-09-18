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

use core::ops;

/// Apply logarithm base two.
#[allow(dead_code)]
pub fn lg<T>(mut val: T) -> Result<T, ()>
    where T: Copy + From<u8> + PartialOrd
             + ops::ShrAssign + ops::BitAnd<Output=T>
             + ops::AddAssign
{
    let mut counter: T = T::from(0);
    while val > T::from(0) && (val & T::from(1)) == T::from(0) {
        counter += T::from(1);
        val >>= T::from(1);
    }
    if val != T::from(1) {
        Err(())
    } else {
        Ok(counter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_input() {
        assert_eq!(lg(0x1).unwrap(), 0);
        assert_eq!(lg(0x2).unwrap(), 1);
        assert_eq!(lg(0x4).unwrap(), 2);
        assert_eq!(lg(0x8).unwrap(), 3);
        assert_eq!(lg(0x8000_0000_0000_0000_u64).unwrap(), 63);
    }

    #[test]
    fn valid_signed_number() {
        assert_eq!(lg(0x1_isize).unwrap(), 0);
        assert_eq!(lg(0x2_isize).unwrap(), 1);
        assert_eq!(lg(0x4_isize).unwrap(), 2);
        assert_eq!(lg(0x8_isize).unwrap(), 3);
        assert_eq!(lg(0x4000_0000_0000_0000_i64).unwrap(), 62);
    }

    #[test]
    #[should_panic]
    fn invalid_input() {
        lg(0x11).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_large_input() {
        lg(0x7000_0000_0000_0000_i64).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_zero() {
        lg(0).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_signed_number() {
        lg(-1).unwrap();
    }
}
