// Copyright (c) 2026 ToPluto Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Basic 32-bit colors based on RGB definition
//! ```rust
//! use crate::drivers::video::colors::*;
//!
//! // Combine channels using the bitwise OR operator (|)
//! let color_fuchsia: u32 = COLOR_RED | COLOR_BLUE;
//!
//! ```

pub const COLOR_RED: u32 = 0xFF_00_00;
pub const COLOR_GREEN: u32 = 0x00_FF_00;
pub const COLOR_BLUE: u32 = 0x00_00_FF;

pub const COLOR_WHITE: u32 = 0xFF_FF_FF;
pub const COLOR_BLACK: u32 = 0x00_00_00;
