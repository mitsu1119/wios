use core::{arch::asm, mem::MaybeUninit};

use cortex_m_semihosting::hprintln;

pub const PS_STACK_SIZE: usize = 1024;

#[repr(align(8))]
pub struct PsStack(MaybeUninit<[u8; PS_STACK_SIZE]>);

#[link_section = ".ps_stack"]
#[no_mangle]
pub static PS_STACK: PsStack = PsStack(MaybeUninit::uninit());

#[repr(C)]
pub struct ContextFrame {
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r12: u32,
    pub lr: u32,
    pub return_address: u32,
    pub xpsr: u32,
}

#[no_mangle]
pub unsafe extern "C" fn process_exec(sp_process: u32) {
    asm!(
        "push {{r4, r5, r6, r7, lr}}",
        "push {{r8, r9, r10, r11}}",
        "msr psp, {sp_process}",
        "svc 0",
        "pop {{r8, r9, r10, r11}}",
        "pop {{r4, r5, r6, r7, lr}}",
        sp_process = in(reg) sp_process,
    );
}

pub extern "C" fn process_main() {
    hprintln!("process main");
    unsafe {
        asm!("svc 0");
    }
    loop {}
}
