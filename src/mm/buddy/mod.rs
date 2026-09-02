// Copyright (c) 2026 ToPluto Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Bare Metal Buddy Allocator
//! Struct and constant values definition

pub mod global_impl;
pub mod implementation;
pub mod loked_buddy_impl;

/// Call to the extern function that calculates the address for the deallocating function
/// in the buddy allocator.
#[cfg(target_arch = "x86_64")]
unsafe extern "sysv64" {
    pub fn ada_compute_buddy_address(
        allocator: *mut BuddyAllocator,
        current_address: *mut *mut u8,
        base_address: u64,
        order: *mut u64,
        page_size: u64,
    );
}

/// Maximum number of orders (levels) managed by the allocator ($0 \dots \text{MAX\_ORDER}-1$).
///
/// With `MAX_ORDER = 11`,
/// the maximum order is 10, corresponding to $2^{10} = 1024$ pages ($4\text{ MiB}$).
pub const MAX_ORDER: usize = 11;

/// Standard memory page size on x86_64 architecture ($4096\text{ bytes}$).
pub const PAGE_SIZE: usize = 4096;

/// An intrusive node stored directly inside free memory blocks.
///
/// Forms a singly linked list for each order inside `free_lists`.
#[repr(C)]
pub struct FreeNode {
	/// Pointer to the next free node in the list.
	pub next: *mut FreeNode,
}

/// Main Buddy Allocator structure (non thread-safe).
#[repr(C)]
pub struct BuddyAllocator {
	/// Array of linked lists of free blocks for each order.
	pub free_lists: [*mut FreeNode; MAX_ORDER],
	/// Starting memory address (physical or virtual) of the heap.
	pub base_addr: u64,
}

/// Thread-safe wrapper for `BuddyAllocator` protected by a spinlock.
pub struct LockedBuddyAllocator(pub spin::Mutex<BuddyAllocator>);
