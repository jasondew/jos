default: run

build:
	cargo build

run: build
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-jos/debug/bootimage-jos.bin
