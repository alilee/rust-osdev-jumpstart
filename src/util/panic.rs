// SPDX-License-Identifier: Unlicense

//! Panic handling.

use crate::cpu;

/// The point of exit for the "standard" (non-testing) `libkernel`.
///
/// This code will be used by the release kernel binary and the `integration tests`. It is linked
/// weakly, so that the integration tests can overload it to exit `QEMU` instead of spinning
/// forever.
#[linkage = "weak"]
#[no_mangle]
#[allow(unreachable_code)]
fn _panic_exit() -> ! {
    #[cfg(target_vendor = "unknown")]
    cpu::qemu_exit_failure();

    cpu::wait_forever();
}

/// Log panic information and abnormal-exit emulator (or hang)
#[cfg(not(test))]
#[allow(unreachable_code)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // use crate::debug::Level;
    //
    // match info.location() {
    //     None => log!(
    //         Level::Fatal,
    //         "Panic: {}",
    //         info.message().unwrap_or(&format_args!("unknown"))
    //     ),
    //     Some(loc) => log!(
    //         Level::Fatal,
    //         "Panic: {} (at {}:{})",
    //         info.message().unwrap_or(&format_args!("unknown")),
    //         loc.file(),
    //         loc.line()
    //     ),
    // };

    _panic_exit();

    cpu::wait_forever();
}
