use core::{arch::asm, mem::MaybeUninit};

use cortex_m_semihosting::hprintln;

pub const PS_STACK_SIZE: usize = 1024;

#[repr(align(8))]
pub struct PsStack(pub MaybeUninit<[u8; PS_STACK_SIZE]>);

#[link_section = ".ps_stack"]
#[no_mangle]
pub static PS_STACK: PsStack = PsStack(MaybeUninit::uninit());

#[link_section = ".ps_stack2"]
#[no_mangle]
pub static PS_STACK2: PsStack = PsStack(MaybeUninit::uninit());

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
pub unsafe extern "C" fn process_exec(sp_process: u32, process_regs: &mut [u32; 8]) -> u32 {
    let mut next_sp: u32;
    asm!(
        // カーネル側のレジスタを退避
        "push {{r4, r5, r6, r7, lr}}",
        "push {{r8, r9, r10, r11}}",
        // プロセスの SP を設定
        "msr psp, {sp_process}",
        // プロセスのレジスタを復元
        "ldmia {process_regs}, {{r4-r11}}",
        "svc 0",
        "stmia {process_regs}, {{r4-r11}}",
        "mrs {next_sp}, psp",
        "pop {{r8, r9, r10, r11}}",
        "pop {{r4, r5, r6, r7, lr}}",
        sp_process = in(reg) sp_process,
        process_regs = in(reg) process_regs,
        next_sp = out(reg) next_sp,
    );
    next_sp
}

pub struct Process {
    sp_process: u32,
    regs: [u32; 8],
}

impl Process {
    pub fn new(ps_stack: &PsStack, ps_main: extern "C" fn() -> !) -> Self {
        let sp_process = ps_stack.0.as_ptr() as u32
            + unsafe { ps_stack.0.assume_init_ref().len() as u32 }
            - 0x20;
        let context_frame: &mut ContextFrame = unsafe { &mut *(sp_process as *mut ContextFrame) };
        context_frame.r0 = 0;
        context_frame.r1 = 0;
        context_frame.r2 = 0;
        context_frame.r3 = 0;
        context_frame.r12 = 0;
        context_frame.lr = 0;
        context_frame.return_address = ps_main as u32;
        context_frame.xpsr = 0x0100_0000;

        Self {
            sp_process,
            regs: [0; 8],
        }
    }

    pub fn exec(&mut self) {
        self.sp_process = unsafe { process_exec(self.sp_process, &mut self.regs) };
    }
}

#[no_mangle]
pub extern "C" fn process_main1() -> ! {
    let mut cnt = 0;
    loop {
        hprintln!("process_main1 {}", cnt);
        cnt += 1;
        unsafe {
            asm!("svc 0");
        }
    }
}

#[no_mangle]
pub extern "C" fn process_main2() -> ! {
    let mut cnt = 0;
    loop {
        hprintln!("process_main2 {}", cnt);
        cnt += 1;
        unsafe {
            asm!("svc 0");
        }
    }
}
