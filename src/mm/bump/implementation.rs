// Copyright (c) 2026 Skidbladnir Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation of the Bare Metal Bump Allocator
//! We move a pointer along the heap, allocating sequentially.

use core::cell::UnsafeCell;
use core::alloc::Layout;
use core::ptr::null_mut;

use crate::mm::bump::definition::BumpAllocator;
use crate::mm::bump::helpers::align_up;

impl BumpAllocator {
    /// Creates a new, uninitialized instance of `BumpAllocator`.
    ///
    /// Initializes all internal memory bounds and the allocation pointer to zero.
    /// The allocator must be initialized with [`init`](Self::init) before any memory 
    /// allocations can be performed safely.
    pub const fn new() -> Self {
        BumpAllocator {
            next: UnsafeCell::new(0),
            heap_start: UnsafeCell::new(0),
            heap_end: UnsafeCell::new(0),
        }
    }

    /// Initializes the heap bounds and allocation pointer using firmware boot metadata.
    ///
    /// # Arguments
    ///
    /// * `boot_info` - Reference to the [`BootInfo`](crate::BootInfo) structure provided
    ///   by the bootloader containing memory layout boundaries (`FrameRange`).
    ///
    /// # Safety
    ///
    /// This function mutates the internal states via raw pointers using [`UnsafeCell`].
    /// It must be called only once during early kernel initialization to avoid race
    /// conditions or corrupting active allocations.
    pub fn init(&self, boot_info: &crate::BootInfo) {
        let start = boot_info.fr.heap_start as usize;
        let end = boot_info.fr.heap_end as usize;

        // SAFETY: We obtain raw mutable pointers to write initial heap boundaries.
        // This is safe provided `init` is called in a single-threaded context during boot.
        unsafe {
            *self.next.get() = start;
            *self.heap_start.get() = start;
            *self.heap_end.get() = end;
        }
    }

    /// Allocates memory with the given size and alignment constraints.
    ///
    /// # Arguments
    ///
    /// * `layout` - Memory layout describing the required `size` and `align`.
    ///
    /// # Returns
    ///
    /// * `*mut u8` - A raw pointer to the allocated memory, or `null_mut()` if:
    ///   - The requested `size` is zero.
    ///   - The heap runs out of memory (Out of Memory).
    ///   - The arithmetic for pointer alignment overflows.
    pub unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // Zero-sized allocations do not require physical memory.
        if size == 0 {
            return null_mut();
        }

        // Dereference internal state safely via UnsafeCell raw pointers.
        unsafe {
            let current_next = *self.next.get();
            let heap_end = *self.heap_end.get();
        

            // Calculate the starting address aligned to requested boundary.
            let alloc_start = align_up(current_next, align);

            // Compute the new `next` pointer after adding requested size.
            let alloc_end = match alloc_start.checked_add(size) {
                Some(end) => end,
                None => return null_mut(), // Overflow protection
            };
    

            // Out of Memory (OOM) check: verify bounds against `heap_end`.
            if alloc_end > heap_end {
                return null_mut(); // Exceeds allocated heap capacity
            }

            // Update the bump pointer boundary.
            *self.next.get() = alloc_end;

            // Return the start address of the newly reserved block.
            alloc_start as *mut u8
        }
    }

    /// Deallocates memory.
    ///
    /// Note: A basic bump allocator cannot free individual blocks of memory.
    /// Memory is only reclaimed when resetting the entire allocator.
    pub unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Intentional no-op: Bump allocators do not support individual deallocations.
    }
}