ARCH=x86_64

.PHONY: all
all: build/disk

.PHONY: qemu
qemu: build/disk
	qemu-system-$(ARCH) -drive file=build/disk,format=raw

.PHONY: debug
debug: build/disk
	qemu-system-$(ARCH) -s -S -drive file=build/disk,format=raw

build/disk: bootloader/*
	mkdir -p build
	nasm -f bin -o $@ -ibootloader/ bootloader/disk.s
