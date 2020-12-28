# rust-osdev-jumpstart
Rust, cargo and QEMU setup for multi-architecture OS development.

## Goal
This repo should give you a boost in starting a bare-metal OS development project in Rust. You can go backwards from here if you just want something working as a starting point (but have different ideas for foundation), or you can go forward and exploit the testing setup.

## Sources
These three are the great Rust bare-metal OS development blogs, and I have borrowed liberally
from their ideas and code.

* [Philipp Oppermann](http://os.phil-opp.com/) - x86_64
* [Andre Richter](https://github.com/rust-embedded/rust-raspi3-tutorial) - Arm
* [Stephen Marz](http://osblog.stephenmarz.com/index.html) - RiscV

## Features

* Doc tests and unit tests run your development host OS for speed.
* Integration tests are run under QEMU with ```cargo test --target <triple>```.
* Integration tests utilise [custom test frameworks](https://rust-lang.github.io/rfcs/2318-custom-test-frameworks.html) feature for convenient test suites.
* QEMU passes back test result to cargo using [qemu_exit](https://crates.io/crates/qemu-exit).
* ```make test_all``` runs doc, unit and integration tests for all three architectures.
* QEMU definitions largely follow the platform-specific blogs above.

## Example usage

```
$ make test_all
cargo test --doc --target=x86_64-apple-darwin
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
   Doc-tests libkernel

running 1 test
test src/lib.rs - (line 4) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.47s

cargo test --lib --target=x86_64-apple-darwin
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running target/x86_64-apple-darwin/debug/deps/libkernel-910fb71f27dff6a6

running 1 test
test test::panics_ok ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

cargo test --target x86_64-unknown-none.json -Zunstable-options -Zbuild-std=core,compiler_builtins -Zbuild-std-features=compiler-builtins-mem
    Finished test [unoptimized + debuginfo] target(s) in 0.04s
     Running target/x86_64-unknown-none/debug/deps/000_kernel_init-f9ceeb28812f2f47
Building bootloader
   Compiling bootloader v0.9.11 (~/.cargo/registry/src/github.com-1ecc6299db9ec823/bootloader-0.9.11)
    Finished release [optimized + debuginfo] target(s) in 1.14s
Running: `qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/deps/bootimage-000_kernel_init-f9ceeb28812f2f47.bin -no-reboot -device isa
     Running target/x86_64-unknown-none/debug/deps/001_should_panic-6a56cc56ef6b6077
Building bootloader
   Compiling bootloader v0.9.11 (~/.cargo/registry/src/github.com-1ecc6299db9ec823/bootloader-0.9.11)
    Finished release [optimized + debuginfo] target(s) in 1.11s
Running: `qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/deps/bootimage-001_should_panic-6a56cc56ef6b6077.bin -no-reboot -device is
cargo test --target riscv64gc-unknown-none-elf
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running target/riscv64gc-unknown-none-elf/debug/deps/000_kernel_init-af9782dbeaca1763
     Running target/riscv64gc-unknown-none-elf/debug/deps/001_should_panic-db251fb771fa34a8
cargo test --target aarch64-unknown-none-softfloat
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running target/aarch64-unknown-none-softfloat/debug/deps/000_kernel_init-13b9d95425b9c0bb
     Running target/aarch64-unknown-none-softfloat/debug/deps/001_should_panic-e72d04dcc4d61eda
```

## Known limitations and extensions

Pull requests welcome!

* Only tested on Mac OS 11 Big Sur.
* Running the doc and unit tests on the host means those tests can't interact with hardware. Even to unit test the architecture-independent code would require either a mock ```_arch/test``` or lots of ```cfg(test)``` peppered through-out. 
* Utilising ```-Zmultitarget``` would be an improvement, but the ```-Zbuild-std``` options required for ```x86_64``` can't be pushed into ```[target.x86_64...]``` inside ```.cargo/config.toml```. You would probably still need ```make``` as a shorthand.
* Logging would be a great inclusion, but might be quite opinionated with quite a bit of serial device code.
* Device tree management in the Makefile for RiscV and Arm.

## Other influences

* [Julia Evans](http://jvns.ca/blog/2014/03/21/my-rust-os-will-never-be-finished/)
* [Bodil Stokke](https://skillsmatter.com/skillscasts/4484-build-your-own-lisp-for-great-justice)
* [Lucas Hartmann](http://interim-os.com/) We follow in his footsteps!
* [Dawid Ciężarkiewicz](https://github.com/dpc/titanos) And Dawid's too!
* [rCoreOS](https://github.com/rcore-os/rCore) Multi-arch
* [Ilya Kartashov](https://lowenware.com/leos/) 
* [Redox OS](https://www.redox-os.org) 