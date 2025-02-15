# SPDX-License-Identifier: BSD-2-Clause-Patent
# SPDX-FileCopyrightText: 2025 System76, Inc.

# Disable built-in rules and variables
MAKEFLAGS += --no-builtin-rules --no-builtin-variables
.SUFFIXES:

export TARGET = x86_64-unknown-uefi
QEMU = qemu-system-x86_64
OVMF = /usr/share/OVMF

.PHONY: build
build:
	cargo build --target $(TARGET) --release

.PHONY: clippy
clippy:
	cargo clippy --target $(TARGET)

.PHONY: clean
clean:
	cargo clean

# TODO: AArch64 options?
.PHONY: qemu
qemu: build
	cp $(OVMF)/OVMF_CODE.fd target/
	cp $(OVMF)/OVMF_VARS.fd target/
	$(QEMU) -enable-kvm -M q35 -m 1024 \
		-vga std -net none \
		-chardev stdio,mux=on,id=debug \
		-device isa-serial,index=2,chardev=debug \
		-device isa-debugcon,iobase=0x402,chardev=debug \
		-drive if=pflash,format=raw,readonly=on,file=target/OVMF_CODE.fd \
		-drive if=pflash,format=raw,readonly=on,file=target/OVMF_VARS.fd \
		-drive format=raw,file=fat:rw:target/$(TARGET)
