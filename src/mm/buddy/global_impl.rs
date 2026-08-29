// Copyright (c) 2026 Skidbladnir Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Contain the Global Implentation of the Buddy Allocator

use crate::mm::buddy::LockedBuddyAllocator;
use crate::mm::buddy::{BuddyAllocator, PAGE_SIZE};

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

unsafe impl GlobalAlloc for LockedBuddyAllocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let size = layout.size().max(layout.align());
		let pages = (size + PAGE_SIZE - 1) / PAGE_SIZE;
		let order = pages.next_power_of_two().trailing_zeros() as usize;

		self.0.lock().alloc(order).unwrap_or(null_mut())
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		let size = layout.size().max(layout.align());
		let pages = (size + PAGE_SIZE - 1) / PAGE_SIZE;
		let order = pages.next_power_of_two().trailing_zeros() as usize;

		unsafe {
			self.0.lock().dealloc(ptr, order);
		}
	}
}

// Concurrency guarantees: mutable access is synchronized by Mutex
unsafe impl Send for BuddyAllocator {}
unsafe impl Send for LockedBuddyAllocator {}
unsafe impl Sync for LockedBuddyAllocator {}
