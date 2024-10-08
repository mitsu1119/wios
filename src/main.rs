#![no_std]
#![no_main]
#![feature(naked_functions)]

#[allow(dead_code)]
mod linked_list;
mod process;
mod reset;
mod svcall;
#[allow(dead_code)]
mod systick;
mod vector_table;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
