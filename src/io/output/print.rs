// Copyright (c) 2026 Skidbladnir Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Main Output Logic Module
//!
//! This module provides the core console output management for the kernel,
//! handling character rendering, string printing, and automatic cursor advancement
//! based on the active font configuration (e.g., bitmap or future vector formats).
//!
//! ## Example Usage
//!
//! ```rust
//! use crate::io::output::Console;
//! use crate::io::output::fonts::{ActualFont, Fonts};
//!
//! // Assuming actual_font and font instances are initialized:
//! // let mut console = Console::init(actual_font, font);
//! // console.print_str("Hello, Skidbladnir Kernel!");
//! ```

use crate::drivers::video::colors::COLOR_WHITE;
use crate::io::output::Console;
use crate::io::output::fonts::{FontVariant, Fonts};

/// Console implementation of output functions.
///
/// Manages screen text placement, cursor tracking, and delegates rendering
/// tasks down to the typography and font subsystems.
impl Console {
	/// Initializes and returns a new [`Console`] instance.
	///
	/// # Arguments
	///
	/// * `actual_font` - The active system typography wrapper configuration.
	/// * `font` - The underlying [`Fonts`] structure containing font data and the framebuffer.
	///
	/// # Returns
	///
	/// Returns a fully configured [`Console`] struct set with default initial coordinates (`x: 20`, `y: 20`).
	///
	/// # Example
	///
	/// ```rust
	/// use crate::io::output::Console;
	/// use crate::io::output::fonts::{ActualFont, Fonts};
	///
	/// // let actual_font = ActualFont { ... };
	/// // let font = Fonts::init(true, fb);
	/// // let console = Console::init(actual_font, font);
	/// ```
	pub fn init(font: Fonts) -> Self {
		Console {
			pos_x: 20,
			pos_y: 20,
			line_number: 0,
			font,
		}
	}

	/// Renders a single ASCII byte onto the console screen and advances the horizontal cursor position.
	///
	/// This method draws the character using the active font layout at the current
	/// `pos_x` and `pos_y` coordinates, automatically calculating the appropriate glyph spacing
	/// (e.g., fixed-width increments for bitmaps vs dynamic width offsets for vector variants).
	///
	/// # Arguments
	///
	/// * `byte` - The raw ASCII byte value (`u8`) to render.
	///
	/// # Example
	///
	/// ```rust
	/// // Print the character 'A' (ASCII code 65)
	/// // console.print_byte(b'A');
	/// ```
	pub fn print_byte(&mut self, byte: u8) {
		// Render the character glyph onto the framebuffer
		self.font
			.print_byte(byte, self.pos_x, self.pos_y, COLOR_WHITE);

		// Determine the horizontal spacing advance width based on the active font variant
		let advance_width = match self.font.variant {
			FontVariant::Bitmap(_) => 8, // Fixed 8-pixel width for standard bitmap fonts
			                             // FontVariant::Vector(ref v) => v.get_glyph_width(byte), // Future vector font implementation
		};

		// Advance the cursor position horizontally
		self.pos_x += advance_width;
	}

	/// Renders an entire text string onto the console screen sequence by sequence.
	///
	/// Iterates through each character of the input string, converting them into bytes
	/// and calling [`Console::print_byte`] consecutively.
	///
	/// # Arguments
	///
	/// * `str` - A string slice (`&str`) containing the text to print.
	///
	/// # Example
	///
	/// ```rust
	/// // Print a welcome banner string to the console
	/// // console.print_str("Welcome to Skidbladnir OS!");
	/// ```
	pub fn print_str(&mut self, str: &str) {
		for char in str.chars() {
			self.print_byte(char as u8);
		}
	}
}
