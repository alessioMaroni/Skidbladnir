// Copyright (c) 2026 Skidbladnir Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![no_std]
#![no_main]

mod panic;
mod boot;

pub use skidbladnir_kernel::{
    BootInfo,
    FrameBufferInfo,
    FrameRange,
};

use uefi::prelude::*;

#[entry]
fn efi_main() -> Status {
    #[cfg(target_arch = "x86_64")]
    let _uefi_status = crate::boot::uefi_boot::boot_uefi();

    main();
}

fn main() -> ! {
    loop {
        core::hint::spin_loop();
    }
}