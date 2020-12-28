// SPDX-License-Identifier: Unlicense

//! Support for integration testing, shared between tests.

/// Point of exit for the integration test runner. By default, completing without panic is success.
///
/// It is linked weakly, so that integration tests can overload it to exit `QEMU` with failure when
/// a panic is the correct behaviour for the test.
#[cfg(target_vendor = "unknown")]
#[linkage = "weak"]
#[no_mangle]
#[allow(unreachable_code)]
fn _test_complete() -> ! {
    crate::cpu::qemu_exit_success();
}

/// The runner for integration tests.
///
/// NOTE: This is not used for unit tests.
#[cfg(target_vendor = "unknown")]
pub fn test_runner(tests: &[&test_types::UnitTest]) {
    // info!("running {} tests", tests.len());
    for test in tests {
        // info!("testing {}", test.name);
        (test.test_func)();
    }
    // info!("test result: ok.");

    _test_complete();
}
