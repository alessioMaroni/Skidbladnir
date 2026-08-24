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
mod mm {
    pub mod buddy{
        pub mod definition;
    }
}

use mm::buddy::definition::ALLOCATOR;

pub use skidbladnir_kernel::{
    BootInfo,
    FrameBufferInfo,
    FrameRange,
};

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
        core::hint::spin_loop();
    }
}