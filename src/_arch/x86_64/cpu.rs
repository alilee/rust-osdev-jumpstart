// SPDX-License-Identifier: Unlicense

//! Code executed on system reset.

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    extern "Rust" {
        fn kernel_init() -> !;
    }
    kernel_init();
}

//--------------------------------------------------------------------------------------------------
// Testing
//--------------------------------------------------------------------------------------------------
use qemu_exit::QEMUExit;

/// Note: io_base must correspond to isa-debug-exit settings Cargo.toml within package.metadata.bootimage
const QEMU_EXIT_HANDLE: qemu_exit::X86 = qemu_exit::X86::new(0xf4, 33);

/// Make the host QEMU binary execute `exit(1)`.
pub fn qemu_exit_failure() -> ! {
    QEMU_EXIT_HANDLE.exit_failure()
}

/// Make the host QEMU binary execute `exit(33)`.
pub fn qemu_exit_success() -> ! {
    QEMU_EXIT_HANDLE.exit_success()
}
