// Copyright (c) 2026 Skidbladnir Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This file contain the Bump Allocator Struct Definitions

use core::cell::UnsafeCell;

/// Bump Allocator Struct Definition
/// A sequential linear allocator (Bump Allocator) designed for early kernel initialization.
///
/// Memory is allocated continuously by advancing a single pointer (`next`) forward.
/// Individual allocations cannot be freed independently; the entire heap is reset at once.
///
/// # Thread Safety
///
/// Uses [`UnsafeCell`] to provide interior mutability, allowing updates via shared
/// references (`&self`). Operations must be synchronized externally if accessed from
/// multiple CPU cores or execution threads.
pub struct BumpAllocator {
    /// The current memory address boundary for the next allocation request.
    ///
    /// Moves forward (bumped) as allocations occur toward `heap_end`.
    pub next: UnsafeCell<usize>,

    /// The base starting physical/virtual address of the heap memory range.
    pub heap_start: UnsafeCell<usize>,

    /// The maximum allowable physical/virtual address limit of the heap memory range.
    pub heap_end: UnsafeCell<usize>,
}