// Copyright (c) 2026 Skidbladnir Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Output Console Module
//!
//! This module provides the core console implementation for the kernel output subsystem,
//! combining active font configurations and line tracking mechanisms.
//! 
//! ```rust
//! use crate::io::output::Console;
//! ```

pub mod fonts;
pub mod print;

use crate::io::output::fonts::{Fonts};

/// Represents the system output console, tracking active typography, current cursor position, 
/// and line tracking metrics.
pub struct Console<'a> {
    /// Fonts struct to output
    pub font: Fonts<'a>,

    /// Tracks the absolute line number or total tracked lines in the console view.
    pub line_number: usize,

    /// pos x
    pub pos_x: usize,
    
    /// pos y
    pub pos_y: usize,
}