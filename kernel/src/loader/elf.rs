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
#![allow(dead_code)]

//! A loader used to load ELF files.

macro_rules! assert_and_shift_bytes {
    {$bytes:expr, $expected:expr} => {{
        if $bytes.len() < $expected.len() {
            return Err(());
        }
        for index in 0..$expected.len() {
            if $bytes[index] != $expected[index] {
                return Err(());
            }
        }
        $bytes = &$bytes[$expected.len()..];
    }};
}

macro_rules! extract_and_shift_bytes {
    {$bytes:expr, $len:expr} => {{
        if $bytes.len() < $len {
            return Err(());
        }
        let mut result: u128 = 0;
        for i in 0..$len {
            result *= 8;
            result += u128::from($bytes[i]);
        }
        $bytes = &$bytes[$len..];
        result
    }};
}

struct FileHeader {
    entry: u64,
    ph_offset: u64,
    sh_offset: u64,
    phentsize: u16,
    phnum: u16,
    shentsize: u16,
    shnum: u16,
    shstrndx: u16,
}

fn parse_file_header(input_bytes: &[u8]) -> Result<FileHeader, ()> {
    let mut bytes = input_bytes;
    assert_and_shift_bytes!(bytes, [0x7f]);
    assert_and_shift_bytes!(bytes, b"ELF");
    // Currently, we support only 64-bit architecture.
    assert_and_shift_bytes!(bytes, [2]);
    // Currently, we support only little-endianness of ELF.
    assert_and_shift_bytes!(bytes, [1]);
    // The only supported version is 1.
    assert_and_shift_bytes!(bytes, [1]);
    // Don't care about the platform.
    assert_and_shift_bytes!(bytes, [0]);
    // Unused 8 bytes.
    bytes = &bytes[8..];
    // We support only ET_EXEC. We don't support dynamic executables yet.
    assert_and_shift_bytes!(bytes, [0x02, 0]);
    // We support only x86-64.
    assert_and_shift_bytes!(bytes, [0x3e, 0]);
    // The only supported version is 1.
    assert_and_shift_bytes!(bytes, [1, 0, 0, 0]);

    let entry = extract_and_shift_bytes!(bytes, 8) as u64;
    let ph_offset = extract_and_shift_bytes!(bytes, 8) as u64;
    let sh_offset = extract_and_shift_bytes!(bytes, 8) as u64;

    // Ignore 4 byte flag.
    bytes = &bytes[4..];
    // The header size must be 64 bytes for x86-64.
    assert_and_shift_bytes!(bytes, [64, 0]);

    let phentsize = extract_and_shift_bytes!(bytes, 2) as u16;
    let phnum = extract_and_shift_bytes!(bytes, 2) as u16;
    let shentsize = extract_and_shift_bytes!(bytes, 2) as u16;
    let shnum = extract_and_shift_bytes!(bytes, 2) as u16;
    #[allow(unused_assignments)]
    let shstrndx = extract_and_shift_bytes!(bytes, 2) as u16;

    Ok(FileHeader {
        entry,
        ph_offset,
        sh_offset,
        phentsize,
        phnum,
        shentsize,
        shnum,
        shstrndx,
    })
}

fn load_segment(input_bytes: &[u8]) -> Result<(), ()> {
    let mut bytes = input_bytes;
    // We support only PT_LOAD.
    assert_and_shift_bytes!(bytes, [1, 0, 0, 0]);

    // TODO: We will ignore the flags for now and implement it later.
    bytes = &bytes[4..];

    let _offset = extract_and_shift_bytes!(bytes, 8) as u64;
    let _address = extract_and_shift_bytes!(bytes, 8) as u64;

    // Ignore suggested physical address.
    bytes = &bytes[8..];

    let _file_size = extract_and_shift_bytes!(bytes, 8) as u64;
    let _mem_size = extract_and_shift_bytes!(bytes, 8) as u64;

    // Ignore alignment.
    #[allow(unused_assignments)]
    bytes = &bytes[8..];

    Ok(())
}

pub fn load_elf(bytes: &[u8]) -> Result<(), ()> {
    let file_header = parse_file_header(bytes)?;

    // The only supported program header entry size is 0x38 bytes because
    // we support only x86-64.
    if file_header.phentsize != 0x38 {
        return Err(());
    }

    // If the blob is not big enough to find a program header table, return
    // an error.
    let phsize = file_header.phentsize * file_header.phnum;
    if (bytes.len() as u64) < file_header.ph_offset + u64::from(phsize) {
        return Err(());
    }

    for i in 0..file_header.phnum {
        let ph_table = &bytes[(file_header.ph_offset as usize)..];
        let phent_start = (i * file_header.phentsize) as usize;
        let phent_end = ((i+1) * file_header.phentsize) as usize;
        let phent = &ph_table[phent_start..phent_end];
        let _ = load_segment(phent);
    }

    Ok(())
}
