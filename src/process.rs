use core::{arch::asm, mem::MaybeUninit};

const PS_STACK_SIZE: usize = 1024;

#[repr(align(8))]
struct PsStack(MaybeUninit<[u8; PS_STACK_SIZE]>);

#[link_section = ".ps_stack"]
#[no_mangle]
static PS_STACK: PsStack = PsStack(MaybeUninit::uninit());

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
