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

//! Unsigned integer module. This module is used for all unsigned integer
//! types. The only difference from the primitive types is that this module
//! internally uses NonZero type to try to optimize the memory layout.

macro_rules! unsigned_integers {
    ( $( $Ty: ident($NonZero: ident, $Uint: ty); )+ ) => {
        $(
            use core::num::$NonZero;

            #[allow(dead_code)]
            #[derive(Eq, PartialEq, Copy, Clone, Debug)]
            pub struct $Ty($NonZero);

            impl $Ty {
                #[allow(dead_code)]
                pub fn new(val: $Uint) -> $Ty {
                    $Ty($NonZero::new(val + 1).unwrap())
                }
                #[allow(dead_code)]
                pub fn get(self) -> $Uint {
                    self.0.get() - 1
                }
            }
        )+
    }
}

unsigned_integers! {
    U8(NonZeroU8, u8);
    U16(NonZeroU16, u16);
    U32(NonZeroU32, u32);
    U64(NonZeroU64, u64);
    U128(NonZeroU128, u128);
    Usize(NonZeroUsize, usize);
}
