// Copyright (c) 2026 ToPluto Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # `x86_64` Context Switching Module
//!
//! Provides architecture-specific routines for switching execution context
//! between kernel tasks on `x86_64`.
//!
//! ## Example
//! ```rust,ignore
//! use crate::arch::x86_64::task::switch_context;
//!
//! unsafe {
//!     switch_context(&mut current_tcb.rsp, &next_tcb.rsp);
//! }
//! ```

use core::arch::global_asm;

// Include the external assembly routine responsible for push/pop of callee-saved registers
// and stack pointer swapping.
global_asm!(include_str!("switch.s"));

unsafe extern "sysv64" {
    /// External assembly routine defined in `switch.s`.
    ///
    /// # Parameters
    /// * `old_rsp` - Pointer to the `rsp` field of the outgoing task's TCB (`*rdi`).
    /// * `new_rsp` - Pointer to the `rsp` field of the incoming task's TCB (`*rsi`).
    fn __switch(old_rsp: *mut u64, new_rsp: *const u64);
}

/// Performs a low-level context switch between two execution contexts.
///
/// Saves the current task's callee-saved registers (`rbp`, `rbx`, `r12`-`r15`) onto its
/// stack, stores the resulting stack pointer in `old_rsp`, replaces `RSP` with `*new_rsp`,
/// and restores the incoming task's state before returning to its `RIP`.
///
/// # Safety
///
/// Calling this function is inherently unsafe because it directly alters CPU state
/// and control flow:
/// - `old_rsp` must be a valid, writable pointer to the memory location storing the current `RSP`.
/// - `new_rsp` must point to a valid stack frame configured with a matching `TaskContext` structure.
/// - Improper stack alignment (must remain 16-byte aligned before `call` boundaries) or corrupted
///   pointers will lead to a kernel panic or CPU triple fault.
pub unsafe fn switch_context(old_rsp: *mut u64, new_rsp: *const u64) {
    unsafe {
        __switch(old_rsp, new_rsp);
    }
}
