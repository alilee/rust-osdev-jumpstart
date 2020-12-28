// SPDX-License-Identifier: Unlicense

//! Processor code.

#[cfg(target_arch = "x86_64")]
#[path = "_arch/x86_64/cpu.rs"]
mod arch_cpu;

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/cpu.rs"]
mod arch_cpu;

#[cfg(target_arch = "riscv64")]
#[path = "_arch/riscv64/cpu.rs"]
mod arch_cpu;

pub use arch_cpu::*;

pub fn wait_forever() -> ! {
    loop {}
}
