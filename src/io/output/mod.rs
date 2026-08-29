// Copyright (c) 2026 Yggdrasil Kernel Project
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

use crate::io::output::fonts::Fonts;

use core::fmt::{self, Write};
use spin::Mutex;

/// Represents the system output console, tracking active typography, current cursor position,
/// and line tracking metrics.
pub struct Console {
	/// Fonts struct to output
	pub font: Fonts,

	/// Tracks the absolute line number or total tracked lines in the console view.
	pub line_number: usize,

	/// pos x
	pub pos_x: usize,

	/// pos y
	pub pos_y: usize,
}

/// Global thread-safe static instance of the kernel output console.
///
/// Protected by a `Mutex` to allow safe interior mutability and synchronized
/// access across different execution contexts (e.g., core logic and interrupts).
/// It is initially set to `None` and populated during early kernel initialization.
pub static WRITER: Mutex<Option<Console>> = Mutex::new(None);

/// Implementation of the core `core::fmt::Write` trait for the `Console` struct.
///
/// This trait integration allows the custom `Console` to bridge with Rust's
/// standard formatting machinery (`core::fmt`), enabling formatted strings
/// to be seamlessly written via standard formatting hooks.
impl Write for Console {
	/// Writes a string slice directly into the console buffer.
	///
	/// # Arguments
	///
	/// * `s` - A string slice (`&str`) containing the text to be rendered.
	///
	/// # Returns
	///
	/// Returns `fmt::Result` indicating success (`Ok(())`) upon completion.
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.print_str(s);
		Ok(())
	}
}

/// Internal helper function invoked by the `print!` and `println!` macros.
///
/// This function locks the global `WRITER` mutex, checks if the console has been
/// successfully initialized, and writes the formatted argument sequence into it.
///
/// # Arguments
///
/// * `args` - A `fmt::Arguments` structure containing the parsed and compiled formatting data.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
	if let Some(console) = WRITER.lock().as_mut() {
		console.write_fmt(args).unwrap();
	}
}

/// Prints formatted text to the kernel console output without a trailing newline.
///
/// This macro wraps around `format_args!` and forwards the payload to the internal
/// `_print` routine. It is exported globally across the crate scope.
///
/// # Example
///
/// ```rust
/// print!("Initializing subsystem: {}", "Memory");
/// ```
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::output::_print(format_args!($($arg)*))
    };
}

/// Prints formatted text to the kernel console output followed by a newline character (`\n`).
///
/// Acts as the kernel-space counterpart to the standard library's `println!` macro.
///
/// # Example
///
/// ```rust
/// println!("Kernel booted successfully at address {:#x}", entry_point);
/// ```
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}
