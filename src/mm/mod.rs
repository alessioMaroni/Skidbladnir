//! Memory Management (MM) Subsystem Module
//!
//! Provides memory allocation primitives for the kernel, including a physical frame/buddy
//! allocator for general kernel dynamics and a lightweight bump allocator for early boot stages.

pub mod buddy {
    pub mod loked_buddy_impl;
    pub mod global_impl;
    pub mod definitions;
    pub mod buddy_impl;
}

 /// Example Usage of the Early Bump Allocator:
/// ```rust
/// // Direct manual allocation via BumpAllocator raw interface
/// let layout = core::alloc::Layout::from_size_align(1024, 8).unwrap();
/// let ptr = unsafe { BUMP_ALLOCATOR.alloc(layout) };
///
/// if !ptr.is_null() {
///     // Access and write to raw allocated memory safely...
/// }
/// ```
pub mod bump {
    pub mod definition;
    pub mod implementation;
    pub mod helpers;
    pub mod global_impl;
}

use crate::mm::buddy::definitions::LockedBuddyAllocator;

/// Primary global kernel heap allocator.
///
/// Handles all dynamic allocations required by standard kernel data structures
/// (`alloc::boxed::Box`, `alloc::vec::Vec`, `alloc::string::String`, etc.).
/// Synchronized via interior locking mechanism to ensure thread safety across CPU cores.
#[global_allocator]
pub static ALLOCATOR: LockedBuddyAllocator = LockedBuddyAllocator::new();