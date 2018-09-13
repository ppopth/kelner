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

/// This is a random number generator used when there is no true randomness,
/// in our system. This will be seeded with 1 by default and will be
/// deterministic when you get a new random number. You can also reseed it
/// any time you want.
pub struct WeakRng {
    seed: u32,
}

impl WeakRng {
    /// Create a new weak random number generator.
    pub fn new() -> WeakRng {
        WeakRng {
            seed: 1,
        }
    }

    /// Reseed the weak random number generator.
    #[allow(dead_code)]
    pub fn reseed(&mut self, seed: u32) {
        self.seed = seed;
    }

    /// Get a new pseudo-random number.
    pub fn next(&mut self) -> u32 {
        self.seed = (self.seed as u64 * 1103515245 + 12345) as u32;
        self.seed
    }
}
