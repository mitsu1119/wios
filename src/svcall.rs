use core::arch::asm;

#[naked]
#[no_mangle]
pub unsafe extern "C" fn svcall() {
    asm!(
        // svcall が呼ばれたのはカーネル or プロセス
        "cmp lr, #0xfffffff9",
        "bne 1f",
        // カーネル -> プロセス
        "mov r0, #1",
        "msr CONTROL, r0",
        "isb",
        "movw lr, #0xfffd",
        "movt lr, #0xffff",
        "bx lr",
        // プロセス -> カーネル
        "1:",
        "mov r0, #0",
        "msr CONTROL, r0",
        "isb",
        "movw lr, #0xfff9",
        "movt lr, #0xffff",
        "bx lr",
        options(noreturn)
    );
}
