// SPDX-License-Identifier: Unlicense

//! Kernel as library, to facilitate integration testing.
//! ```rust,should_panic
//! assert!(false);
//! ```

#![no_std]
#![feature(linkage)] // for weak linkage of panic::_panic_exit
#![feature(naked_functions)] // for _reset
#![feature(asm)] // used throughout archs

pub mod cpu;
pub mod util;

#[cfg(test)]
mod test {
    #[test]
    #[should_panic]
    fn panics_ok() {
        assert!(false);
    }
}
