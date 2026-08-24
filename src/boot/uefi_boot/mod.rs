// Copyright (c) 2026 Skidbladnir Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub mod fbi;

use crate::boot::uefi_boot::fbi::get_framebuffer_info;

use uefi::{
	Status,
	boot::{self, AllocateType, MemoryType},
	mem::memory_map::MemoryMap,
	prelude::*,
	proto::{
		media::{
			file::{File, FileAttribute, FileInfo, FileMode, FileType},
			fs::SimpleFileSystem,
		},
	},
};

/// Specifies the physical base address (2 MB) designated for loading and executing the kernel.
///
/// This address is utilized for several critical architectural reasons:
///
/// 1. **Deterministic Entry Point:** Both the bootloader and the kernel linker script
///    must align on the exact memory layout post-load, ensuring a valid transition
///    via pointer transmute to the kernel entry routine.
///
/// 2. **Memory Protection & Reserved Regions:** The lower megabyte of physical memory
///    is typically allocated for legacy structures, BIOS/UEFI firmware data, and system vectors.
///    Locating the kernel at 2 MB (`0x200000`) avoids region overlap and ensures a contiguous
///    available address space.
///
/// 3. **Symbol Integrity:** Synchronization with the linker script parameters guarantees
///    that absolute symbol references for functions and global data remain valid,
///    preventing architectural faults or execution failures during initialization.
#[allow(dead_code)]
const KERNEL_PHYS_BASE: u64 = 0x200000;

pub fn boot_uefi() -> Status {
    // Initializes the `uefi-rs` library support utilities.
	//
	// This performs fundamental setup operations for the bare-metal environment:
	//
	// * **Standard Output Configuration:** Hooks up `uefi::println!` to the Graphics Output
	//   Protocol (GOP) or system serial console, allowing us to display log messages.

	// * **Allocator & Panic Setup:** Prepares basic tools necessary for formatting macros
	//   and utility functions to operate correctly in an OS-less UEFI context.
	uefi::helpers::init().unwrap();

	uefi::println!("Welgome to Skidbladnir Kernel Project's Bootloader");

	// Retrieve the UEFI memory map to analyze system RAM.
	// We iterate through all memory descriptors to determine the lowest accessible
	// physical address, the highest address, and the total usable (conventional) memory.
	let memory_map = boot::memory_map(MemoryType::LOADER_DATA).expect("Failed to get memory map");

	let mut min_phys_addr = u64::MAX;
	let mut max_phys_addr = 0u64;
	let mut total_conventional_bytes = 0u64;

	for entry in memory_map.entries() {
		let start = entry.phys_start;
		let end = start + (entry.page_count * 4096);

		if start < min_phys_addr {
			min_phys_addr = start;
		}

		if entry.ty == MemoryType::CONVENTIONAL {
			if end > max_phys_addr {
				max_phys_addr = end;
			}
			total_conventional_bytes += entry.page_count * 4096;
		}
	}

	// Locate and mount the boot partition's Simple File System.
	// This protocol is necessary to access files residing on the FAT32 EFI System Partition.
	let sfs_handle = boot::get_handle_for_protocol::<SimpleFileSystem>()
		.expect("Failed to find SimpleFileSystem");

	let mut sfs = boot::open_protocol_exclusive::<SimpleFileSystem>(sfs_handle)
		.expect("Failed to open SimpleFileSystem");

	let mut root = sfs.open_volume().expect("Failed to open root");

	// Attempt to open the compiled kernel binary at the root of the EFI partition.
	let file_handle = root
		.open(
			cstr16!("kernel.bin"),
			FileMode::Read,
			FileAttribute::empty(),
		)
		.expect("Failed to find kernel.bin");

	let mut regular_file = match file_handle.into_type().unwrap() {
		FileType::Regular(f) => f,
		_ => panic!("The kernel is not a regular file"),
	};

	let mut info_buffer = [0u8; 256];
	let file_info = regular_file
		.get_info::<FileInfo>(&mut info_buffer)
		.expect("Failed to get file info");

	let kernel_file_size = file_info.file_size();

	// Determine the number of 4KB UEFI pages required to hold the kernel binary.
	// We explicitly allocate these pages at `KERNEL_PHYS_BASE` to ensure the kernel
	// resides exactly where its linker script expects it to be.
	let pages_to_allocate = ((kernel_file_size + 4095) / 4096) as usize;
	let kernel_size_ram = (pages_to_allocate * 4096) as u64;

	let kernel_mem_ptr = boot::allocate_pages(
		AllocateType::Address(KERNEL_PHYS_BASE),
		MemoryType::LOADER_DATA,
		pages_to_allocate,
	)
	.expect("Page allocation at 2MB failed");

	// Map the allocated physical memory as a mutable slice and read the kernel
	// binary data directly from the disk into this memory region.
	let kernel_raw_ptr = kernel_mem_ptr.as_ptr() as *mut u8;

	let kernel_buffer =
		unsafe { core::slice::from_raw_parts_mut(kernel_raw_ptr, kernel_size_ram as usize) };

	let bytes_read = regular_file.read(kernel_buffer).expect("Kernel read error");

	// Construct the BootInfo structure to pass critical hardware and layout
	// information to the kernel. The kernel's heap is configured to start immediately
	// after the loaded kernel binary and extend to the end of usable RAM.
	let heap_start = KERNEL_PHYS_BASE + kernel_size_ram;
	let heap_end = max_phys_addr;

	let fbi = match get_framebuffer_info() {
		Ok(info) => info,
		Err(status) => {
			uefi::println!("Impossibile inizializzare il Framebuffer GOP: {:?}", status);
			panic!("Errore GOP fatale");
		}
	};

	let boot_info = crate::BootInfo {
		kernel_file_size: bytes_read as u64,
		kernel_size_ram,

		fr: crate::FrameRange {
			ram_start: min_phys_addr,
			ram_end: max_phys_addr,
			total_conventional_bytes,
			heap_start,
			heap_end,
		},

		fbi,
	};

	uefi::println!("--- BootInfo Summary ---");

	uefi::println!("--- FrameRange ---");
	uefi::println!("	kernel_base: {:#x}", KERNEL_PHYS_BASE);
	uefi::println!("	ram_start: {:#x}", boot_info.fr.ram_start);
	uefi::println!("	ram_end: {:#x}", boot_info.fr.ram_end);
	uefi::println!(
		"	total_conventional: {} MB",
		boot_info.fr.total_conventional_bytes / (1024 * 1024)
	);
	uefi::println!("	heap_start: {:#x}", boot_info.fr.heap_start);
	uefi::println!("	heap_end: {:#x}", boot_info.fr.heap_end);
	uefi::println!("	------------------------");
	uefi::println!("--- FrameBufferInfo ---");
	uefi::println!("	base_address: {:#x}", boot_info.fbi.base_address);
	uefi::println!("	buffer_size: {:#x}", boot_info.fbi.buffer_size);
	uefi::println!("	width: {:#x}", boot_info.fbi.width);
	uefi::println!("	height: {:#x}", boot_info.fbi.height);
	uefi::println!("	stride: {:#x}", boot_info.fbi.stride);

	uefi::println!("kernel_file_size: {} bytes", boot_info.kernel_file_size);
	uefi::println!("kernel_size_ram: {} bytes", boot_info.kernel_size_ram);
	uefi::println!("------------------------");

	boot::stall(core::time::Duration::from_secs(2));

	uefi::println!("Jumping...");

	// Terminate UEFI Boot Services.
	// This yields full control of the hardware to our OS. After this point,
	// UEFI services (like file I/O or standard screen printing) are no longer available.
	let _final_memory_map = unsafe { boot::exit_boot_services(Some(MemoryType::LOADER_DATA)) };

	// Construct a function pointer to the kernel's physical base address.
	type KernelEntry = unsafe extern "sysv64" fn(&'static crate::BootInfo) -> !;
	let entry_point: KernelEntry = unsafe { core::mem::transmute(KERNEL_PHYS_BASE) };

	// Place the `BootInfo` struct in physical memory right after the kernel binary.
	// This ensures the data persists safely and can be read by the kernel.
	let boot_info_ptr = (KERNEL_PHYS_BASE + bytes_read as u64) as *mut crate::BootInfo;
	unsafe {
		core::ptr::write(boot_info_ptr, boot_info);
	}
	let static_boot_info = unsafe { &*boot_info_ptr };

	// Execute the final jump into the kernel entry point, passing the BootInfo reference.
	// There is no return from this point forward.
	unsafe {
		entry_point(static_boot_info);
	}
}