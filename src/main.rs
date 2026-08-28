// Copyright (c) 2026 Skidbladnir Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![no_std]
#![no_main]
#![allow(unused_features)]
#![feature(asm_experimental_arch)]
#![allow(dead_code)]

extern crate alloc;
#[allow(unused_imports)]
use alloc::vec::Vec;
#[allow(unused_imports)]
use alloc::string::String;

mod panic;
mod boot;
mod arch;
mod mm;
mod drivers;

use mm::ALLOCATOR;

/// Contains metadata about the graphical framebuffer passed from the UEFI
/// Graphics Output Protocol (GOP) during the boot handoff.
///
/// **Usage:** Used by the kernel's early display driver or terminal emulator
/// to draw pixels, text, and graphics directly to the screen before a full
/// GPU driver is initialized.
#[repr(C)]
#[derive(Debug)]
pub  struct FrameBuffer {
	/// The physical or virtual base memory address of the linear framebuffer.
	///
	/// **Calculation:** Retrieved directly from the UEFI GOP mode info structure
	/// (`FrameBufferBase`).
	/// **Usage:** The starting point for writing raw pixel data into video memory.
	pub base_address: u64,

	/// The total size of the framebuffer memory buffer in bytes.
	///
	/// **Calculation:** Retrieved from the UEFI GOP mode info structure (`FrameBufferSize`).
	/// **Usage:** Used to verify bounds and ensure memory safety when drawing to the screen.
	pub buffer_size: u64,

	/// The horizontal resolution of the screen in pixels.
	///
	/// **Calculation:** Provided by UEFI GOP (`Resolution[0]`).
	/// **Usage:** Defines the maximum valid X coordinate ($x < \text{width}$) for rendering.
	pub width: u32,

	/// The vertical resolution of the screen in pixels.
	///
	/// **Calculation:** Provided by UEFI GOP (`Resolution[1]`).
	/// **Usage:** Defines the maximum valid Y coordinate ($y < \text{height}$) for rendering.
	pub height: u32,

	/// The number of pixels per scanline (also known as pitch or stride).
	///
	/// **Calculation:** Provided by UEFI GOP. Note that stride can sometimes be
	/// greater than or equal to `width` due to hardware padding requirements.
	/// **Usage:** Essential for calculating the correct memory offset of a pixel:
	/// `base_address + (y * stride + x) * bytes_per_pixel`.
	pub stride: u32,
}

/// Contains boundaries and statistics regarding physical memory and the early heap.
///
/// **Usage:** Helps the kernel understand available RAM layout and sets up boundaries
/// for the early physical memory manager and bump allocator.
#[repr(C)]
#[derive(Debug)]
pub struct FrameRange {
	/// The lowest physical memory address available on the system.
	///
	/// **Calculation:** Derived by iterating through the UEFI memory map and
	/// finding the absolute minimum `phys_start` across all memory entries.
	/// **Usage:** Used by the kernel to understand the absolute floor of physical RAM.
	pub ram_start: u64,

	/// The highest physical memory address of usable (conventional) memory.
	///
	/// **Calculation:** Derived by finding the maximum `phys_start + (page_count * 4096)`
	/// specifically among memory regions marked as `MemoryType::CONVENTIONAL`.
	/// **Usage:** Used by the kernel's physical memory manager to establish the
	/// upper boundary of usable RAM space.
	pub ram_end: u64,

	/// The total amount of usable RAM available to the operating system, in bytes.
	///
	/// **Calculation:** The sum of `page_count * 4096` for all UEFI memory map
	/// entries marked as `MemoryType::CONVENTIONAL`.
	/// **Usage:** Used for system statistics (e.g., displaying total RAM) and
	/// sizing internal kernel data structures like page frame arrays.
	pub total_conventional_bytes: u64,

	/// The safe starting physical address for the kernel's dynamic memory allocator.
	///
	/// **Calculation:** Computed as `KERNEL_PHYS_BASE + kernel_size_ram`. It points
	/// to the first byte immediately following the loaded kernel binary.
	/// **Usage:** Crucial for the initial bump allocator. It guarantees the kernel
	/// will not accidentally overwrite its own code or data when allocating memory.
	pub heap_start: u64,

	/// The maximum physical address up to which the initial kernel allocator can grow.
	///
	/// **Calculation:** Set to `max_phys_addr` (same as `ram_end`).
	/// **Usage:** Acts as the out-of-memory boundary for early boot allocations
	/// before a full virtual memory manager and page allocator are initialized.
	pub heap_end: u64,
}

/// Contains critical hardware, memory layout, and environment data passed
/// from the UEFI bootloader to the kernel during the handoff process.
///
/// Because the kernel assumes control after UEFI boot services are terminated,
/// it cannot query the firmware for this information itself. This structure
/// serves as the kernel's definitive map of physical memory and its own
/// placement within it.
#[repr(C)]
#[derive(Debug)]
pub struct BootInfo {
	/// The exact size of the kernel binary file as it was stored on the disk.
	///
	/// **Calculation:** Extracted directly from the FAT32 EFI partition using
	/// the UEFI `FileInfo` protocol before reading the file.
	/// **Usage:** Mostly informational, useful for debugging and verifying that
	/// the entire file was read successfully.
	pub kernel_file_size: u64,

	/// The total footprint of the kernel in physical RAM, aligned to 4KB page boundaries.
	///
	/// **Calculation:** `((kernel_file_size + 4095) / 4096) * 4096`. This rounds up
	/// the raw file size to the nearest multiple of a UEFI memory page (4KB).
	/// **Usage:** Ensures that the `heap_start` begins on a clean page boundary,
	/// preventing unaligned memory access and protecting the trailing bytes of the kernel page.
	pub kernel_size_ram: u64,

	/// Physical memory ranges, layout boundaries, and early heap limits.
	///
	/// **Calculation:** Gathered by parsing the UEFI memory map immediately before
	/// exiting boot services.
	/// **Usage:** Provides the foundational data structures needed to configure
	/// memory management subsystems.
	pub fr: FrameRange,

	/// Graphical framebuffer configuration details.
	///
	/// **Calculation:** Obtained by querying the UEFI Graphics Output Protocol (GOP).
	/// **Usage:** Passed to the display subsystem to enable early text and graphics output.
	pub fb: FrameBuffer,
}


#[cfg(target_arch = "x86_64")]
unsafe extern "sysv64" {
    pub fn ada_sum_integer(a: i32, b: i32) -> i32;
}

#[cfg(target_os = "uefi")]
#[uefi::prelude::entry]
fn efi_main() -> uefi::Status {
    #[cfg(target_arch = "x86_64")]
    let boot_info = crate::boot::uefi_boot::boot_uefi();

    kernel_main(&boot_info);
}

pub fn kernel_main(_boot_info: &BootInfo) -> ! {
    arch::x86_64::init::init_x86_64();

    ALLOCATOR.init(_boot_info);

    let mut my_vec: Vec<String> = Vec::new();
    my_vec.push(String::from("Hello"));

    _boot_info.fb.set_pixel(200, 200, 0xFF_FF_FF);
    
    loop {

    }
    
}