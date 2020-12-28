// SPDX-License-Identifier: Unlicense

//! Change behaviour of QEMU exit code on panic

extern crate libkernel;
use libkernel::cpu;

/// Overwrites libkernel's `panic::_panic_exit()` with success version.
#[allow(unreachable_code)]
#[no_mangle]
fn _panic_exit() -> ! {
    cpu::qemu_exit_success();
}

/// Point of exit for the integration test runner. Completing without panic is failure.
#[allow(unreachable_code)]
#[no_mangle]
fn _test_complete() -> ! {
    cpu::qemu_exit_failure();
}
