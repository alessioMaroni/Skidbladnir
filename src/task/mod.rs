// Copyright (c) 2026 ToPluto Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//!

use alloc::vec::Vec;
use core::arch:asm;

const DEFAULT_STACK_SIZE: usize = 64 * 1024; // 64 KB

#[repr(C)]
pub struct TaskContext {
    pub rsp: u64,
}

pub enum TaskState {
    Ready,
    Running,
    Stopped,
    Dead,
}

pub struct Task {
    pub id: u64,
    pub context: TaskContext,
    pub stack: Vec<u8>,
    pub state: TaskState,
}
