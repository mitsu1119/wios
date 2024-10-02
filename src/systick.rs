use core::ptr::{read_volatile, write_volatile};

use cortex_m_semihosting::hprintln;

#[no_mangle]
pub extern "C" fn systick() {
    hprintln!("SysTick");
}

// address
const SYST_CSR: usize = 0xE000_E010;
const SYST_RVR: usize = 0xE000_E014;
const SYST_CVR: usize = 0xE000_E018;

// flags
const SYST_CSR_COUNTFLAG: u32 = 1 << 3;
const SYST_CSR_CLKSOURCE: u32 = 1 << 2;
const SYST_CSR_TICKINT: u32 = 1 << 1;
const SYST_CSR_ENABLE: u32 = 1;

#[inline]
fn read_csr() -> u32 {
    unsafe { read_volatile(SYST_CSR as *mut u32) }
}

#[inline]
pub fn enable() {
    unsafe {
        write_volatile(SYST_CSR as *mut u32, read_csr() | SYST_CSR_ENABLE);
    }
}

#[inline]
pub fn disable() {
    unsafe { write_volatile(SYST_CSR as *mut u32, read_csr() & !SYST_CSR_ENABLE) }
}

#[inline]
pub fn enable_interrupt() {
    unsafe {
        write_volatile(SYST_CSR as *mut u32, read_csr() | SYST_CSR_TICKINT);
    }
}

#[inline]
pub fn disable_interrupt() {
    unsafe {
        write_volatile(SYST_CSR as *mut u32, read_csr() & !SYST_CSR_TICKINT);
    }
}

#[inline]
pub fn set_rvr(value: u32) {
    unsafe {
        write_volatile(SYST_RVR as *mut u32, value);
    }
}

#[inline]
pub fn clear_cvr() {
    unsafe {
        write_volatile(SYST_CVR as *mut u32, 0);
    }
}
