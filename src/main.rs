#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(unsafe_cell_from_mut)]

#[allow(dead_code)]
mod linked_list;
#[allow(dead_code)]
mod peripheral;
mod pin;
#[allow(dead_code)]
mod port;
#[allow(dead_code)]
mod port_reg;
mod process;
#[allow(dead_code)]
mod register;
mod reset;
mod scheduler;
mod svcall;
#[allow(dead_code)]
mod systick;
mod vector_table;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
