// Copyright (c) 2019, Suphanat Chunhapanya
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

//! System call module. This module includes all system call routines.

// The parameter that it needs here should be ExceptionStackFrame in crate
// x86_64, but we don't want to add another depency crate. So we decided to
// put a dummy parameter here.
#[cfg(not(test))]
pub extern "x86-interrupt"
fn interrupt_handler(_: &mut ()) {
    println!("Interrupted");
}
