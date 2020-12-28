HOST = $(shell rustc -Vv | grep host: | awk '/host: / {print $$2; exit}')

BINTOOLS = rust
OBJCOPY = $(BINTOOLS)-objcopy
OBJDUMP = $(BINTOOLS)-objdump

# This would be better with -Zmultitarget, except that unstable.build-std can't be set per target in .cargo/config.toml.
x86_64_target = --target x86_64-unknown-none.json -Zunstable-options -Zbuild-std=core,compiler_builtins -Zbuild-std-features=compiler-builtins-mem
riscv64_target = --target riscv64gc-unknown-none-elf
aarch64_target = --target aarch64-unknown-none-softfloat

doc_test:
	cargo test --doc --target=$(HOST)

unit_test: doc_test
	cargo test --lib --target=$(HOST)

build_all:
	cargo bootimage $(x86_64_target)
	cargo build $(riscv64_target)
	cargo build $(aarch64_target)

define KERNEL_TEST_RUNNER
#!/usr/local/bin/fish

$(OBJCOPY) -O binary $$argv[2] $$argv[2].bin

switch $$argv[1]
	case aarch64
		qemu-system-aarch64 -M virt -cpu cortex-a53 -m 256M -nographic -semihosting -kernel $$argv[2].bin > $$argv[2].out
	case riscv64
		qemu-system-riscv64 -M virt -cpu rv64 -smp 1 -m 256M -nographic -d guest_errors,unimp -bios none -kernel $$argv[2] > $$argv[2].out
	case x86_64
		echo Error: Handled by bootimage runner.
end

set result $$status

if test $$result -ne 0
#	cat $$argv[2].out
#	$(OBJDUMP) -dS $$argv[2] > $$argv[2].code
#	$(OBJDUMP) -d $$argv[2] > $$argv[2].s
end

exit $$result
endef

export KERNEL_TEST_RUNNER
target/kernel_test_runner.sh: Makefile
	@mkdir -p target
	@echo "$$KERNEL_TEST_RUNNER" > target/kernel_test_runner.sh
	@chmod +x target/kernel_test_runner.sh

test_runner: target/kernel_test_runner.sh

test_all: unit_test test_runner
	cargo test $(x86_64_target)
	cargo test $(riscv64_target)
	cargo test $(aarch64_target)

qemu_x86_64: build_all
	qemu-system-x86_64 -m 256M -nographic -s -S -device loader,file=target/x86_64-unknown-none/debug/kernel

gdb_x86_64: build_all
	gdb -iex 'target remote localhost:1234'

qemu_aarch64: build_all
	qemu-system-aarch64 -M virt -cpu cortex-a53 -m 256M -nographic -semihosting -s -S -kernel target/aarch64-unknown-none-softfloat/debug/kernel

gdb_aarch64: build_all
	gdb -iex 'file target/aarch64-unknown-none-softfloat/debug/kernel' -iex 'target remote localhost:1234'

qemu_riscv64: build_all
	qemu-system-riscv64 -machine virt -cpu rv64 -smp 4 -m 256M -nographic -serial mon:stdio -bios none -device virtio-rng-device -device virtio-gpu-device -device virtio-net-device -device virtio-tablet-device -device virtio-keyboard-device -s -S -kernel target/riscv64gc-unknown-none-elf/debug/kernel

gdb_riscv64: build_all
	gdb -iex 'set architecture riscv:rv64' -iex 'target remote localhost:1234'
