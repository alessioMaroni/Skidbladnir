// Copyright (c) 2026 ToPluto Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Font Logic Module
//!
//! This module manages font selection, typography configuration, and low-level character
//! rendering for the ToPluto kernel's output subsystem. It provides robust abstractions
//! to switch between different font rendering engines
//! (such as bitmap matrices and future vector outlines)
//! and draws characters directly onto the target framebuffer.
//!
//! ## Example Usage
//!
//! ```rust
//! use crate::io::output::fonts::{Fonts, ActualFont, FontVariant};
//! use crate::FrameBuffer;
//!
//! // 1. Create or obtain a mock/real Framebuffer instance
//! let fb = FrameBuffer::new(); // Assumes a constructor exists for testing/usage
//!
//! // 2. Initialize the font structure specifying bitmap usage
//! let mut font_subsystem = Fonts::init(true, fb);
//!
//! // 3. Print a character ('A') at screen coordinates (10, 20) with a white color (0xFFFFFF)
//! font_subsystem.print_byte(b'A', 10, 20, 0xFFFFFF);
//! ```

mod bitmap;

use crate::FrameBuffer;
use crate::io::output::fonts::bitmap::FONT_ALPHABET;

/// Represents the active system-wide font configuration container.
///
/// Wraps around the inner [`Fonts`] structure to expose typography settings
/// to the main console and display drivers.
pub struct ActualFont {
	/// The underlying font management and rendering structure.
	pub font: Fonts,
}

/// Defines the supported typography variants available within the kernel.
///
/// This enum allows dynamic dispatch and pattern matching over different
/// font rendering implementations during text output operations.
pub enum FontVariant {
	/// Standard bitmap-based font alphabet containing an 8x8 pixel grid layout
	/// for up to 26 uppercase/lowercase alphanumeric characters.
	Bitmap([[u8; 8]; 26]),
}

/// Manages active font state, typography options, and direct pixel rendering.
///
/// The `Fonts` structure acts as the primary controller for mapping characters
/// to visual pixels via an associated [`FrameBuffer`].
pub struct Fonts {
	/// The currently active font engine and data variant.
	pub variant: FontVariant,

	/// The target framebuffer reference used for low-level pixel manipulation.
	pub fb: FrameBuffer,
}

impl Fonts {
	/// Initializes and returns a new [`Fonts`] instance based on user configuration.
	///
	/// # Arguments
	///
	/// * `use_bitmap` - A boolean flag specifying whether to enable the bitmap font engine (`true`)
	///   or fall back/switch to alternative configurations (`false`).
	/// * `fb` - The target [`FrameBuffer`] instance where pixels will be drawn.
	///
	/// # Returns
	///
	/// Returns a fully configured [`Fonts`] struct ready for glyph rendering operations.
	///
	/// # Example
	///
	/// ```rust
	/// use crate::io::output::fonts::Fonts;
	/// use crate::FrameBuffer;
	///
	/// let fb = FrameBuffer::new();
	/// let font_manager = Fonts::init(true, fb);
	/// ```
	pub fn init(use_bitmap: bool, fb: FrameBuffer) -> Self {
		let variant = if use_bitmap {
			// Select the standard bitmap font variant
			FontVariant::Bitmap(FONT_ALPHABET)
		} else {
			// Fallback layout variant when bitmap configuration is disabled
			FontVariant::Bitmap(FONT_ALPHABET)
		};

		Self { variant, fb }
	}

	/// Matches an input ASCII byte to its corresponding 8-bit bitmap glyph representation.
	///
	/// Inspects the active font variant. If the font is configured as a bitmap,
	/// it maps standard ASCII characters (both uppercase and lowercase letters)
	/// to their respective 8-byte row definitions inside the glyph array.
	///
	/// # Arguments
	///
	/// * `byte` - An ASCII character represented as a raw `u8` byte value.
	///
	/// # Returns
	///
	/// Returns `Some([u8; 8])` containing the 8-row byte matrix if the character
	/// is supported, or `None` if the character is out of bounds or unsupported.
	pub fn match_ascii_to_font(&self, byte: u8) -> Option<[u8; 8]> {
		match &self.variant {
			FontVariant::Bitmap(alphabet) => match byte {
				b'A'..=b'Z' => Some(alphabet[(byte - b'A') as usize]),
				b'a'..=b'z' => Some(alphabet[(byte - b'a') as usize]),
				_ => None,
			},
		}
	}

	/// Renders a single ASCII byte onto the framebuffer using the loaded bitmap font data.
	///
	/// This method retrieves the 8x8 pixel glyph for the given character, iterates through
	/// each row and column bit, and calls [`FrameBuffer::set_pixel`] for every active pixel bit.
	///
	/// # Arguments
	///
	/// * `byte` - The target ASCII character (`u8`) to render.
	/// * `start_x` - The initial horizontal screen coordinate (X axis offset in pixels).
	/// * `start_y` - The initial vertical screen coordinate (Y axis offset in pixels).
	/// * `color` - The 32-bit color value to apply to the drawn character pixels.
	///
	/// # Example
	///
	/// ```rust
	/// use crate::io::output::fonts::Fonts;
	/// use crate::FrameBuffer;
	///
	/// let fb = FrameBuffer::new();
	/// let mut font_manager = Fonts::init(true, fb);
	///
	/// // Draw an 'S' at position (0, 0) in red (0xFF0000)
	/// font_manager.print_byte(b'S', 0, 0, 0xFF0000);
	/// ```
	pub fn print_byte(&mut self, byte: u8, start_x: usize, start_y: usize, color: u32) {
		if let Some(glyph) = self.match_ascii_to_font(byte) {
			// Iterate through each of the 8 vertical rows of the glyph matrix
			for (row_idx, row_byte) in glyph.iter().enumerate() {
				// Iterate through each bit (columns 0 to 7) within the row byte
				for col_idx in 0..8 {
					// Check if the pixel bit is set (non-zero)
					if (row_byte & (1 << (7 - col_idx))) != 0 {
						// Calculate absolute screen coordinates,
						// safely casting usize to u32
						let x = (start_x + col_idx) as u32;
						let y = (start_y + row_idx) as u32;

						// Draw the individual pixel
						// onto the framebuffer hardware/buffer
						self.fb.set_pixel(x, y, color);
					}
				}
			}
		}
	}
}
