// Copyright (c) 2026 Skidbladnir Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! System Documentation: Boot Subsystem (UEFI Protocol)
//!
//! This module implements the boot handshake with UEFI firmware, scans physical
//! memory topology, configures the graphic framebuffer, and permanently disables
//! UEFI Boot Services.

pub mod fbi;

use crate::BootInfo;
use self::fbi::get_framebuffer_info;
use uefi::boot::{self, MemoryType};
use uefi::mem::memory_map::MemoryMap;

/// Initializes the hardware environment via UEFI and performs the transition to bare-metal Ring 0.
///
/// # Architecture and Execution Phases
///
/// This function serves as the entry point for the boot subsystem, executing a sequential
/// initialization through four distinct phases:
///
/// 1. **Runtime Helpers**: Initializes `uefi-rs` formatting utilities and panic handlers.
/// 2. **Memory Discovery**: Scans firmware Memory Map descriptors to compute physical RAM address
///    boundaries and available conventional memory.
/// 3. **GOP Resolution**: Queries the *Graphics Output Protocol* to retrieve the linear Framebuffer 
///    physical base address and display dimensions.
/// 4. **Firmware Detachment**: Invokes `exit_boot_services()`, permanently invalidating firmware 
///    drivers and transferring exclusive control to the kernel.
///
/// # Safety Invariants
///
/// - **Point of No Return**: Once `boot::exit_boot_services()` completes, invoking any UEFI service
///   functions (e.g., `uefi::println!`, `boot::stall`, or allocators) will immediately trigger a 
///   **Page Fault (#PF)** or hardware lockup.
/// - **Memory Ownership**: The memory map read prior to exit represents a static snapshot;
///   subsequent RAM management is handed off entirely to the kernel's physical allocator.
///
/// # Returns
///
/// Returns a [`BootInfo`] structure containing physical RAM layout and GOP configuration.
pub fn boot_uefi() -> BootInfo {
    // Phase 1: Initialize runtime helpers for console I/O and panics.
    uefi::helpers::init().unwrap();
    uefi::println!("Welcome to Skidbladnir Kernel!");

    // Phase 2: Retrieve the firmware memory map for physical RAM discovery.
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

    // Phase 3: Initialize display device via GOP (Graphics Output Protocol).
    let fbi = get_framebuffer_info().expect("Fatal error: GOP Framebuffer initialization failed");

    // Construct boot metadata payload for the kernel.
    let boot_info = BootInfo {
        kernel_file_size: 0,
        kernel_size_ram: 0,
        fr: crate::FrameRange {
            ram_start: min_phys_addr,
            ram_end: max_phys_addr,
            total_conventional_bytes,
            heap_start: 0,
            heap_end: max_phys_addr,
        },
        fbi,
    };

    // Diagnostic log to UEFI console prior to shutting down boot services.
    uefi::println!("--- FrameBufferInfo ---");
    uefi::println!("    base_address: {:#x}", boot_info.fbi.base_address);
    uefi::println!("    width: {}", boot_info.fbi.width);
    uefi::println!("    height: {}", boot_info.fbi.height);

    // Stall to allow serial/console output buffer flush.
    boot::stall(core::time::Duration::from_secs(1));
    uefi::println!("Exiting Boot Services...");

    /*
    unsafe {
        let risultato_ada: i32 = crate::ada_sum_integer(25, 25);
        uefi::println!("--- Ada Integration Test ---");
        uefi::println!("    Risultato somma Ada (25 + 25): {}", risultato_ada);
    }
    */

    // Phase 4: Terminate Boot Services.
    // SAFETY: After this call, UEFI-provided resources are no longer accessible.
    let _final_memory_map = unsafe { 
        boot::exit_boot_services(Some(MemoryType::LOADER_DATA)) 
    };

    boot_info
}