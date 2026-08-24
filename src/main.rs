#![no_std]
#![no_main]

mod panic;
mod boot;

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

    // Il firmware UEFI non esiste più: entriamo nel kernel senza mai restituire il controllo
    kernel_main(&boot_info);
}

pub fn kernel_main(_boot_info: &BootInfo) -> ! {
    // Qui sei in bare-metal puro.
    // Puoi scrivere sul Framebuffer disegnando direttamente sui pixel all'indirizzo `boot_info.fbi.base_address`.
    loop {
        core::hint::spin_loop();
    }
}