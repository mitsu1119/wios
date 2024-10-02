#![no_std]
#![no_main]

mod reset;
mod systick;
mod vector_table;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
