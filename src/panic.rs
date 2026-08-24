// Copyright (c) 2026 Skidbladnir Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg(not(test))]
#[allow(unused_variables)]
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    // TODO: Once io module is implemented print the panic info
    loop {

    }
}