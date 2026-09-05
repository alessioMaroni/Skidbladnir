// Copyright (c) 2026 ToPluto Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// TODO: Document adll the ada directory

#![no_std]
#![no_main]
#![allow(unused_features)]
#![feature(asm_experimental_arch)]
#![allow(dead_code)]

mod arch;
mod boot;
mod drivers;
mod io;
mod mm;
mod panic;
mod sync;
mod task;
mod boot_info;

use mm::ALLOCATOR;

pub(crate) use boot_info::*;

extern crate alloc;
#[allow(unused_imports)]
use alloc::string::String;
#[allow(unused_imports)]
use alloc::vec::Vec;
#[allow(unused_imports)]
use alloc::string::ToString;

use crate::io::output::Console;
use crate::io::output::fonts::Fonts;

#[cfg(target_arch = "x86_64")]
unsafe extern "sysv64" {
	pub fn ada_sum_integer(a: i32, b: i32) -> i32;
}

#[cfg(target_os = "uefi")]
#[uefi::prelude::entry]
fn efi_main() -> uefi::Status {
	#[cfg(target_arch = "x86_64")]
	let mut boot_info = crate::boot::uefi_boot::boot_uefi();

	kernel_main(&mut boot_info);
}

pub fn kernel_main(_boot_info: &mut BootInfo) -> ! {
	ALLOCATOR.init(_boot_info);

	let font_manager = Fonts::init(true, _boot_info.fb);

	let console = Console::init(font_manager, 0x0);

	console.clear_console();

	*crate::io::output::WRITER.lock() = Some(console);

	let mut my_vec: Vec<String> = Vec::new();
    my_vec.push(String::from("Hello from Vector"));
    my_vec.push(String::from("Hello from Vector"));

    println!("{:?}", my_vec);

    loop {}
}

