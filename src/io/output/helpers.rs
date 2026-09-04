// Copyright (c) 2026 ToPluto Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # helpers.rs
//! 
//! ```rust
//! use crate::io::output::helpers;
//! ```
//!
//! Implement helper functions for the 'Console' structure

use crate::io::output::Console;

impl Console {
    /// Clears the entire framebuffer by filling it with the configured background color byte value.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `self.base_address` points to a valid, mapped memory
    /// buffer that remains accessible for at least `self.buffer_size` bytes.
    pub fn clear_console(&self) {
        unsafe {
            let ptr = self.font.fb.base_address as *mut u8;
            core::ptr::write_bytes(ptr, self.background as u8, self.font.fb.buffer_size as usize);
        }
    }
}
