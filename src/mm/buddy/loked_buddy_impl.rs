// Copyright (c) 2026 Skidbladnir Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Loked Buddy Allocator Implementation

use crate::mm::buddy::definitions::{BuddyAllocator, LockedBuddyAllocator};

impl LockedBuddyAllocator {
	/// Creates an uninitialized thread-safe allocator instance at compile time.
	pub const fn new() -> Self {
		Self(spin::Mutex::new(BuddyAllocator::new()))
	}

	/// Initializes the protected allocator by acquiring the spinlock.
	pub fn init(&self, boot_info: &crate::BootInfo) {
		self.0.lock().init(boot_info);
	}
}
