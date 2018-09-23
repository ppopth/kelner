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
#![allow(dead_code)]

const LIST_SIZE: usize = 0x100;

#[derive(Copy, Clone)]
struct Interval {
    start: usize,
    end: usize,
}

struct IntervalList {
    list: [Option<Interval>; LIST_SIZE],
    len: usize,
}

impl IntervalList {
    pub fn new() -> IntervalList {
        IntervalList {
            list: [None; LIST_SIZE],
            len: 0,
        }
    }
    pub fn from(_bytes: &[u8]) -> IntervalList {
        IntervalList {
            list: [None; LIST_SIZE],
            len: 0,
        }
    }
}
