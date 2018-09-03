# Copyright (c) 2018, Suphanat Chunhapanya
# This file is part of Kelner.
#
# Kelner is free software; you can redistribute it and/or
# modify it under the terms of the GNU General Public License
# as published by the Free Software Foundation; either version 2
# of the License, or (at your option) any later version.
#
# Kelner is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with Kelner.  If not, see <https://www.gnu.org/licenses/>.
ARCH=x86_64

.PHONY: all
all: build/disk

.PHONY: clean
clean:
	rm -rf build target

.PHONY: qemu
qemu: build/disk
	qemu-system-$(ARCH) -drive file=$<,format=raw

.PHONY: debug
debug: build/diskdev
	qemu-system-$(ARCH) -s -S -drive file=$<,format=raw & \
	gdb && fg

build/disk: build/kernel bootloader/*
	mkdir -p build
	nasm -f bin -o $@ \
		-D KERNEL_FILE=../$< -ibootloader/ \
		-D ENTRY_POINT=$(shell objdump -f target/release/kernel | \
			grep "start address" | cut -d ' ' -f 3) \
		bootloader/disk.s

build/diskdev: build/kerneldev bootloader/*
	mkdir -p build
	nasm -f bin -o $@ \
		-D KERNEL_FILE=../$< -ibootloader/ \
		-D ENTRY_POINT=$(shell objdump -f target/debug/kernel | \
			 grep "start address" | cut -d ' ' -f 3) \
		bootloader/disk.s

build/kernel: $(shell find kernel -type f) Cargo.toml
	mkdir -p build
	cargo build --release
	objcopy -O binary -S target/release/kernel $@

build/kerneldev: $(shell find kernel -type f) Cargo.toml
	mkdir -p build
	cargo build
	objcopy -O binary -S target/debug/kernel $@
