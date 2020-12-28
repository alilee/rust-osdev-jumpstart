// SPDX-License-Identifier: Unlicense

#![no_std]
#![no_main]

extern crate libkernel; // yes, this is needed

#[allow(unused_imports)]
use libkernel::*;

#[no_mangle]
unsafe fn kernel_init() -> ! {
    loop {}
}
