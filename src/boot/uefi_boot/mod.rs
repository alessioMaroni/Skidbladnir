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
pub mod fr;

use crate::BootInfo;
use self::fbi::get_framebuffer_info;
use self::fr::get_frame_range;

use uefi::boot::{self, MemoryType};

pub const KERNEL_ENTRY: usize = 0x00200000;

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

    let fr = get_frame_range().expect("fatal error: Frame Range initialization failed");

    // Phase 3: Initialize display device via GOP (Graphics Output Protocol).
    let fbi = get_framebuffer_info().expect("Fatal error: GOP Framebuffer initialization failed");

    // Construct boot metadata payload for the kernel.
    let boot_info = BootInfo {
        kernel_file_size: 0,
        kernel_size_ram: 0,
        fr,
        fbi,
    };

    // Diagnostic log to UEFI console prior to shutting down boot services.
    uefi::println!("=== SKIDBLADNIR BOOT INFO ===");
    uefi::println!("Kernel File Size: {} bytes", boot_info.kernel_file_size);
    uefi::println!("Kernel RAM Size:  {} bytes", boot_info.kernel_size_ram);
    
    uefi::println!("\n--- Frame Range ---");
    uefi::println!("RAM Start:                {:#018x}", boot_info.fr.ram_start);
    uefi::println!("RAM End:                  {:#018x}", boot_info.fr.ram_end);
    uefi::println!("Total Conventional Bytes: {} MB", boot_info.fr.total_conventional_bytes / (1024 * 1024));
    uefi::println!("Heap Start:               {:#018x}", boot_info.fr.heap_start);
    uefi::println!("Heap End:                 {:#018x}", boot_info.fr.heap_end);

    uefi::println!("\n--- Frame Buffer Info ---");
    uefi::println!("Base Address: {:#018x}", boot_info.fbi.base_address);
    uefi::println!("Buffer Size:  {} bytes", boot_info.fbi.buffer_size);
    uefi::println!("Resolution:   {}x{}", boot_info.fbi.width, boot_info.fbi.height);
    uefi::println!("Stride:       {} pixels", boot_info.fbi.stride);
    uefi::println!("=============================");

    // Stall to allow serial/console output buffer flush.
    boot::stall(core::time::Duration::from_secs(1));
    uefi::println!("Exiting Boot Services...");

    // Phase 4: Terminate Boot Services.
    // SAFETY: After this call, UEFI-provided resources are no longer accessible.
    let _final_memory_map = unsafe { 
        boot::exit_boot_services(Some(MemoryType::LOADER_DATA)) 
    };

    boot_info
}