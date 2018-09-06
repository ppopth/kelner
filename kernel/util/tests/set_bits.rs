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

extern crate util;
use util::set_bits;

#[test]
fn valid_input() {
    let input: u8 = 0b11111111;
    assert_eq!(set_bits(input, 2, 5, 0b010), 0b11101011);
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

