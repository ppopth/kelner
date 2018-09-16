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

//! Debugging module. This module is used for debugging purpose only.
//! You can print any string on the screen using this module.

/// The maximum number of columns on the screen.
const NUM_COLUMNS: usize = 80;
/// The maximum number of lines on the screen.
const NUM_LINES: usize = 25;

/// The structure containing details of the screen.
struct Screen {
    x: usize,
    y: usize,
}

static mut SCREEN: Screen = Screen {
    x: 0,
    y: 0,
};

/// Print a string with a new line.
pub fn println(s: &str) {
    print(s);
    printnl();
}

/// Print a string.
pub fn print(s: &str) {
    for &byte in s.as_bytes().iter() {
        printc(byte);
    }
}

/// Print a new line.
pub fn printnl() {
    unsafe {
        SCREEN.x = 0;
        SCREEN.y += 1;
        if SCREEN.y == NUM_LINES {
            SCREEN.y = 0;
        }
    }
}

/// Print one character.
pub fn printc(byte: u8) {
    unsafe {
        let position = SCREEN.y * NUM_COLUMNS + SCREEN.x;
        let vga_buffer = 0xb8000 as *mut u8;

        *vga_buffer.offset(position as isize * 2) = byte;
        *vga_buffer.offset(position as isize * 2 + 1) = 0xb;

        SCREEN.x += 1;
        if SCREEN.x == NUM_COLUMNS {
            SCREEN.x = 0;
            SCREEN.y += 1;
        }
        if SCREEN.y == NUM_LINES {
            SCREEN.y = 0;
        }
    }
}
