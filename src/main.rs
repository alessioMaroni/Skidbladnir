#![no_std]
#![no_main]

mod panic;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    log::info!("Welcome to Skidbladnir Kernel Project");
    
    loop {

    }
}