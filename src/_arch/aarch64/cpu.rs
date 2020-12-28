// SPDX-License-Identifier: Unlicense

//! Code executed on system reset.

#[link_section = ".startup"]
#[no_mangle]
#[naked]
/// Entry point for OS
///
/// Positioned at magic address by linker.ld.
///
/// NOTE: must not use stack before SP set.
pub unsafe extern "C" fn _start() -> ! {
    #[allow(dead_code)]
    extern "Rust" {
        fn kernel_init() -> !;
    }

    asm!(
        "   mov     x1, 0x50000000 // top of 256MB memory
            mov     sp, x1
            b       kernel_init",
        options(noreturn)
    );
}

//--------------------------------------------------------------------------------------------------
// Testing
//--------------------------------------------------------------------------------------------------
use qemu_exit::QEMUExit;

pub const QEMU_EXIT_HANDLE: qemu_exit::AArch64 = qemu_exit::AArch64::new();

/// Make the host QEMU binary execute `exit(1)`.
pub fn qemu_exit_failure() -> ! {
    QEMU_EXIT_HANDLE.exit_failure()
}

/// Make the host QEMU binary execute `exit(0)`.
pub fn qemu_exit_success() -> ! {
    QEMU_EXIT_HANDLE.exit_success()
}
