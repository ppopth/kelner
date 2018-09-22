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

use core::fmt;

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

impl Screen {
    /// Print one character.
    fn putc(&mut self, byte: u8) {
        if byte == b'\n' {
            self.y += 1;
            self.x = 0;
            if self.y == NUM_LINES {
                self.y = 0;
            }
            return;
        }
        unsafe {
            let position = self.y * NUM_COLUMNS + self.x;
            let vga_buffer = 0xb8000 as *mut u8;

            *vga_buffer.offset(position as isize * 2) = byte;
            *vga_buffer.offset(position as isize * 2 + 1) = 0xb;

            self.x += 1;
            if self.x == NUM_COLUMNS {
                self.x = 0;
                self.y += 1;
            }
            if self.y == NUM_LINES {
                self.y = 0;
            }
        }
    }
}

impl fmt::Write for Screen {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for c in s.bytes() {
            self.putc(c);
        }
        Ok(())
    }
}

pub fn write(args: fmt::Arguments) -> Result<(), fmt::Error> {
    unsafe {
        fmt::write(&mut SCREEN, args)?;
    }
    Ok(())
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::debug::write(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => ({
        let _ = print!("{}\n", format_args!($($arg)*));
    })
}
