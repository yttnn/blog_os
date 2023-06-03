IMAGE = target/x86_64-blog_os/debug/bootimage-blog_os.bin

.PHONY: all
all: build run

.PHONY: run
run: build
	qemu-system-x86_64 -drive format=raw,file=${IMAGE} -monitor stdio

.PHONY: build
build:
	cargo build