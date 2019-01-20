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

//! If you want to see more detail about each field, look at Intel
//! Architectures Software Developer Manual: Vol 3.

#[macro_export]
macro_rules! idt_entry {
    {$(.$field:ident = $value:expr),*} => {{
        // The inital configuration for interrupt gate.
        let mut result: u128 = 0b00110 << 40;
        $(
            match stringify!($field) {
                "offset"      => {
                  result = set_bits(result, 0, 16, $value & ((1<<16)-1));
                  result = set_bits(result, 48, 64, $value >> 16);
                  result = set_bits(result, 64, 96, $value >> 32);
                },
                "selector"    => result = set_bits(result, 16, 32, $value),
                "d"           => result = set_bits(result, 43, 44, $value),
                "dpl"         => result = set_bits(result, 45, 47, $value),
                "p"           => result = set_bits(result, 47, 48, $value),
                _ => (),
            }
        );*
        result
    }};
}
