// Copyright (c) 2026 Yggdrasil Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This file contain the logic to extract Frame Range Info data

use uefi::boot::{self, MemoryType};
use uefi::mem::memory_map::MemoryMap;

/// Get the Frame Range
pub fn get_frame_range() -> Result<crate::FrameRange, uefi::Status> {
	let memory_map = boot::memory_map(MemoryType::LOADER_DATA)
		.expect("Fatal error: failed to retrieve UEFI memory map");

	let mut min_phys_addr = u64::MAX;
	let mut max_phys_addr = 0u64;
	let mut total_conventional_bytes = 0u64;

	// Iteratively scan descriptors provided by the firmware.
	for entry in memory_map.entries() {
		let start = entry.phys_start;
		let end = start + (entry.page_count * 4096);

		// Identify the lower physical memory boundary.
		if start < min_phys_addr {
			min_phys_addr = start;
		}

		// Compute usable RAM (MemoryType::CONVENTIONAL) and upper physical limit.
		if entry.ty == MemoryType::CONVENTIONAL {
			if end > max_phys_addr {
				max_phys_addr = end;
			}
			total_conventional_bytes += entry.page_count * 4096;
		}
	}

	Ok(crate::FrameRange {
		ram_start: min_phys_addr,
		ram_end: max_phys_addr,
		total_conventional_bytes,
		heap_start: crate::boot::uefi_boot::KERNEL_ENTRY as u64,
		heap_end: max_phys_addr,
	})
}
