// Copyright (c) 2026 Yggdrasil Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Bare-Metal Buddy Allocator
//!
//! This module implements a **Buddy Allocator** for dynamic heap memory management
//! in the *Yggdrasil* kernel.
//!
//! ## Algorithm
//! The allocator divides memory into blocks with sizes equal to powers of two ($2^k \times \text{PAGE\_SIZE}$).
//! - **Allocation (`alloc`):** Searches for an available block of the requested order. If none is available,
//!   it recursively splits a higher-order block in half into two "buddies".
//! - **Deallocation (`dealloc`):** Calculates the buddy address using an XOR operation on the offset and,
//!   if the buddy is free, recursively merges (*coalesces*) the two blocks into a higher-order block.

use crate::mm::buddy::{BuddyAllocator, FreeNode, MAX_ORDER, PAGE_SIZE};

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
		if order >= MAX_ORDER {
			return None;
		}

		for current_order in order..MAX_ORDER {
			if !self.free_lists[current_order].is_null() {
				// Fetch the first available block from the current order
				let block = self.free_lists[current_order];
				unsafe {
					self.free_lists[current_order] = (*block).next;
				}

				// Recursively split down to the requested order
				let mut size = (1 << current_order) * PAGE_SIZE;
				for j in (order..current_order).rev() {
					size /= 2;
					let buddy = (block as usize + size) as *mut FreeNode;
					unsafe {
						(*buddy).next = self.free_lists[j];
						self.free_lists[j] = buddy;
					}
				}

				return Some(block as *mut u8);
			}
		}

		None
	}

	/// Frees a previously allocated block and attempts recursive merging (coalescing).
	///
	/// # Safety
	/// - `ptr` must point to a block allocated by this allocator.
	/// - `order` must match the exact order used during allocation.
	pub unsafe fn dealloc(&mut self, ptr: *mut u8, mut order: usize) {
		let mut current_addr = ptr as usize;
		let base = self.base_addr as usize;

		// Calculate buddy address and attempt merging with higher orders
		while order < MAX_ORDER - 1 {
			let block_offset = current_addr - base;
			let buddy_offset = block_offset ^ ((1 << order) * PAGE_SIZE);
			let buddy_addr: usize = base + buddy_offset;

			if self.remove_from_freelist(order, buddy_addr as *mut FreeNode) {
				// Buddy found and removed from free_list: merge into the block with the lower address
				current_addr = core::cmp::min(current_addr, buddy_addr);
				order += 1;
			} else {
				break;
			}
		}

		// Insert the (possibly coalesced) block into the free_list of the reached order
		let node = current_addr as *mut FreeNode;
		unsafe {
			(*node).next = self.free_lists[order];
		}
		self.free_lists[order] = node;
	}

	/// Removes a specific target node from the free list of the specified order.
	///
	/// Used during deallocation to extract the buddy prior to merging.
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
}
