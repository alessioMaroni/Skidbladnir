// Copyright (c) 2026 ToPluto Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Bare-Metal Buddy Allocator
//!
//! This module implements a **Buddy Allocator** for dynamic heap memory management
//! in the *ToPluto* kernel.
//!
//! ## Algorithm
//! The allocator divides memory into blocks with sizes equal to powers of two
//!     ($2^k \times \text{PAGE\_SIZE}$).
//! - **Allocation (`alloc`):** Searches for an available block of the requested order.
//!     If none is available,
//!     it recursively splits a higher-order block in half into two "buddies".
//! - **Deallocation (`dealloc`):**
//!      Calculates the buddy address using an XOR operation on the offset and,
//!      if the buddy is free, recursively merges the two blocks into a higher-order block.

use crate::mm::buddy::{BuddyAllocator, FreeNode, MAX_ORDER, PAGE_SIZE};
use crate::mm::buddy::{ada_compute_buddy_address, ada_search_free_block};

use core::ptr::null_mut;

impl BuddyAllocator {
	/// Creates an uninitialized allocator instance.
	///
	/// Can be evaluated at compile time (`const fn`).
	pub const fn new() -> Self {
		Self {
			free_lists: [null_mut(); MAX_ORDER],
			base_addr: 0,
		}
	}

	/// Initializes the allocator with the base heap address provided by the `BootInfo` structure.
	///
	/// Inserts the initial memory block into the highest order level of the allocator.
	///
	/// # Safety
	/// The address `boot_info.fr.heap_start` must point to a valid, mapped RAM region.
	pub fn init(&mut self, boot_info: &crate::BootInfo) {
		self.base_addr = boot_info.fr.heap_start;

		let node = self.base_addr as *mut FreeNode;

		unsafe {
			(*node).next = null_mut();
		}

		self.free_lists[MAX_ORDER - 1] = node;
	}

	/// Allocates a contiguous memory block equal to $2^{\text{order}}$ pages.
	///
	/// # Parameters
	/// - `order`: The requested order size ($0 \le \text{order} < \text{MAX\_ORDER}$).
	///
	/// # Returns
	/// Returns a `*mut u8` pointer to the allocated block, or `None` if memory is insufficient.
	pub fn alloc(&mut self, order: usize) -> Option<*mut u8> {
        let order_u64 = order as u64;

        // Call the Ada backend to find and split a free block
        let ptr = unsafe {
            ada_search_free_block(self as *mut Self, order_u64)
        };

        if ptr.is_null() {
            None
        } else {
            Some(ptr)
        }
    }

	/// Frees a previously allocated block and attempts recursive merging (coalescing).
	///
	/// # Safety
	/// - `ptr` must point to a block allocated by this allocator.
	/// - `order` must match the exact order used during allocation.
	pub unsafe fn dealloc(&mut self, ptr: *mut u8, order: usize) {
        let mut current_ptr = ptr;
        let mut order_u64 = order as u64;

        // Delegates buddy calculation and recursive coalescing to Ada
        unsafe {
            ada_compute_buddy_address(
                self as *mut Self,
                &mut current_ptr,
                self.base_addr,
                &mut order_u64,
                PAGE_SIZE as u64,
            );
        }

        let final_order = order_u64 as usize;

        // Insert the (possibly coalesced) block into the free_list of the reached order
        let node = current_ptr as *mut FreeNode;
        unsafe {
            (*node).next = self.free_lists[final_order];
        }
        self.free_lists[final_order] = node;
    }

	// Removes a specific target node from the free list of the specified order.
	//
	// Used during deallocation to extract the buddy prior to merging.
	/*
	fn remove_from_freelist(&mut self, order: usize, target: *mut FreeNode) -> bool {
		let mut curr = &mut self.free_lists[order];
		while !curr.is_null() {
			if *curr == target {
				unsafe {
					*curr = (**curr).next;
				}
				return true;
			}
			unsafe {
				curr = &mut (**curr).next;
			}
		}
		false
	}
	*/
}
