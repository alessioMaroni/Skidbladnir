// Copyright (c) 2026 Yggdrasil Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This file olds the principals init function to setup the x86_64 envirorment

/// Initializes the x86_64 environment for the Yggdrasil Kernel.
///
/// This function performs low-level hardware setup using inline assembly.
/// Specifically, it interacts with I/O port addresses to configure the serial port (UART)
/// settings, such as setting the baud rate divisor and clearing line control registers
/// for early kernel logging and debugging.
///
/// # Safety
/// This function is marked `unsafe` because it executes raw inline assembly
/// (`core::arch::asm!`), directly reading and writing to hardware I/O ports (`0x3F8`, `0x3F9`, `0x3FB`).
/// Improper port manipulation can cause undefined behavior or hardware instability.
pub fn init_x86_64() {
	unsafe {
		core::arch::asm!(
			// Configure serial port baud rate and line control
			"mov dx, 0x3F9",
			"xor al, al",
			"out dx, al",

			"mov dx, 0x3FB",
			"mov al, 0x80",
			"out dx, al",

			"mov dx, 0x3F8",
			"mov al, 0x03",
			"out dx, al",

			"mov dx, 0x3F9",
			"xor al, al",
			"out dx, al",

			"mov dx, 0x3FB",
			"mov al, 0x03",
			"out dx, al",

			out("ax") _,
			out("dx") _,
		);
	}
}
