#![cfg(not(test))]
#[allow(unused_variables)]
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    // TODO: Once io module is implemented print the panic info
    log::info!("Kernel Panic: {:?}", info);
    loop {

    }
}