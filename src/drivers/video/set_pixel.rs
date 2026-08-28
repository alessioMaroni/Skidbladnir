// Copyright (c) 2026 Skidbladnir Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Basic single pixel output function.
//! set position and color
//! 
//! # Module
//! ```rust
//! use crate::drivers::video::set_pixel::*;
//! ```

use crate::drivers::video::colors::COLOR_BLACK;

/// Sets the color of a pixel directly in the framebuffer.
///
/// # Arguments
/// * `x` - X coordinate of the pixel.
/// * `y` - Y coordinate of the pixel.
/// * `color` - Numerical color value to write.
/// * `fb` - Reference to the [`crate::FrameBufferInfo`] struct containing screen metadata.
///
/// # Safety
/// This function performs direct memory writes using raw pointers (`*mut u32`) 
/// inside an `unsafe` block.
/// 
/// # Example
/// ```rust
/// use crate::drivers::video::colors::*;
/// use crate::drivers::video::set_pixel::set_pixel;
/// use crate::FrameBufferInfo;
/// 
/// let color = COLOR_RED as u32;
/// let pos_x = 200 as u32;
/// let pos_y = 200 as u32;
/// 
/// set_pixel(pos_x, pos_y, color, frame_buffer);
/// ```
#[inline(always)]
pub fn set_pixel(x: u32, y: u32, color: u32, fb: &crate::FrameBufferInfo) {
    if x > fb.width || y > fb.height {
        return;
    }

    let offset = ((y * fb.width) + x) as usize;
    let base_addr: *mut u32 = fb.base_address as *mut u32;

    unsafe {
        base_addr.add(offset).write_volatile(color);
    }
}

/// Clears a single pixel by setting its color to black (`COLOR_BLACK`).
///
/// # Arguments
/// * `x` - X coordinate of the pixel to clear.
/// * `y` - Y coordinate of the pixel to clear.
/// * `fb` - Reference to the [`crate::FrameBufferInfo`] struct.
/// 
/// # Example
/// ```rust
/// use crate::drivers::video::set_pixel::clear_pixel;
/// use crate::FrameBufferInfo;
/// 
/// let pos_x = 200 as u32;
/// let pos_y = 200 as u32;
/// 
/// clear_pixel(pos_x, pos_y, frame_buffer);
/// ```
pub fn clear_pixel(x: u32, y: u32, fb: &crate::FrameBufferInfo) {
    set_pixel(x, y, COLOR_BLACK, fb);
}