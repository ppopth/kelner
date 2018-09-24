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
macro_rules! cr3 {
    {$(.$field:ident = $value:expr),*} => {{
        let mut result: u64 = 0;
        $(
            match stringify!($field) {
                "write_through" => result = set_bits(result, 3, 4, $value),
                "cache_disable" => result = set_bits(result, 4, 5, $value),
                "address" => result = set_bits(result, 12, MAXPHYADDR, $value),
                _ => (),
            }
        );*
        result
    }};
}

#[macro_export]
macro_rules! page_directory_entry {
    {$(.$field:ident = $value:expr),*} => {{
        let mut result: u64 = 0;
        $(
            match stringify!($field) {
                "present"       => result = set_bits(result, 0, 1, $value),
                "write"         => result = set_bits(result, 1, 2, $value),
                "supervisor"    => result = set_bits(result, 2, 3, $value),
                "write_through" => result = set_bits(result, 3, 4, $value),
                "cache_disable" => result = set_bits(result, 4, 5, $value),
                "accessed"      => result = set_bits(result, 5, 6, $value),
                "address" => result = set_bits(result, 12, MAXPHYADDR, $value),
                "exe_disable"   => result = set_bits(result, 63, 64, $value),
                _ => (),
            }
        );*
        result
    }};
}

#[macro_export]
macro_rules! page_table_entry {
    {$(.$field:ident = $value:expr),*} => {{
        // Because this is a final page, we need to set bit 7 (PS) to be 1.
        let mut result: u64 = (1 << 7);
        $(
            match stringify!($field) {
                "present"       => result = set_bits(result, 0, 1, $value),
                "write"         => result = set_bits(result, 1, 2, $value),
                "supervisor"    => result = set_bits(result, 2, 3, $value),
                "write_through" => result = set_bits(result, 3, 4, $value),
                "cache_disable" => result = set_bits(result, 4, 5, $value),
                "accessed"      => result = set_bits(result, 5, 6, $value),
                "dirty"         => result = set_bits(result, 6, 7, $value),
                "address" => result = set_bits(result, 12, MAXPHYADDR, $value),
                "exe_disable"   => result = set_bits(result, 63, 64, $value),
                _ => (),
            }
        );*
        result
    }};
}
