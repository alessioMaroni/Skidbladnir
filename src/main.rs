// Copyright (c) 2026 Skidbladnir Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![no_std]
#![no_main]
#![allow(unused_features)]
#![feature(asm_experimental_arch)]
#![allow(dead_code)]

extern crate alloc;
#[allow(unused_imports)]
use alloc::vec::Vec;
#[allow(unused_imports)]
use alloc::string::String;

mod panic;
mod boot;
mod arch;
mod mm;

use mm::ALLOCATOR;

pub use skidbladnir_kernel::{
    BootInfo,
    FrameBufferInfo,
    FrameRange,
};

#[cfg(target_arch = "x86_64")]
unsafe extern "sysv64" {
    pub fn ada_sum_integer(a: i32, b: i32) -> i32;
}

#[cfg(target_os = "uefi")]
#[uefi::prelude::entry]
fn efi_main() -> uefi::Status {
    #[cfg(target_arch = "x86_64")]
    let boot_info = crate::boot::uefi_boot::boot_uefi();

    kernel_main(&boot_info);
}

pub fn kernel_main(_boot_info: &BootInfo) -> ! {
    arch::x86_64::init::init_x86_64();

    ALLOCATOR.init(_boot_info);

    let mut my_vec: Vec<String> = Vec::new();
    my_vec.push(String::from("Hello"));

    loop {

    }
    
}