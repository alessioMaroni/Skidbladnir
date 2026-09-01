// Copyright (c) 2026 ToPluto Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This file contain the logic to extract Frame Buffer Info data

use uefi::proto::console::gop::GraphicsOutput;

/// Queries the UEFI Graphics Output Protocol (GOP) to retrieve the active
/// graphical framebuffer details and configuration.
///
/// # Returns
/// * `Ok(FrameBuffer)` containing the physical base address, size, dimensions,
///   and stride of the current display mode.
/// * `Err(uefi::Status)` if the GOP protocol handle cannot be found, opened,
///   or if querying the mode/framebuffer fails.
///
/// # Errors
/// This function will return an error if:
/// * The system does not support or expose the UEFI `GraphicsOutput` protocol.
/// * Opening the protocol exclusively fails due to firmware state or locking.
pub fn get_framebuffer_info() -> Result<crate::FrameBuffer, uefi::Status> {
	let gop_handle =
		uefi::boot::get_handle_for_protocol::<GraphicsOutput>().map_err(|e| e.status())?;

	let mut gop = uefi::boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle)
		.map_err(|e| e.status())?;

	let mode_info = gop.current_mode_info();
	let mut fb = gop.frame_buffer();

	Ok(crate::FrameBuffer {
		base_address: fb.as_mut_ptr() as u64,
		buffer_size: fb.size() as u64,
		width: mode_info.resolution().0 as u32,
		height: mode_info.resolution().1 as u32,
		stride: mode_info.stride() as u32,
	})
}
