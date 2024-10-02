use core::ptr::{self, addr_of, addr_of_mut};

use cortex_m_semihosting::hprintln;

use crate::systick;

#[no_mangle]
pub unsafe extern "C" fn reset() {
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;
        static mut _sidata: u8;
        static mut _sdata: u8;
        static mut _edata: u8;
    }

    // .bss 領域のゼロ初期化
    let bss_len = addr_of!(_ebss) as usize - addr_of!(_sbss) as usize;
    ptr::write_bytes(addr_of_mut!(_sbss), 0, bss_len);

    // ram から初期値を読み出し .data領域の初期化
    let data_len = addr_of!(_edata) as usize - addr_of!(_sdata) as usize;
    ptr::copy_nonoverlapping(addr_of!(_sidata), addr_of_mut!(_sdata), data_len);

    hprintln!("Hello World!");

    // SysTick initialize
    hprintln!("SysTick init");
    systick::enable();
    systick::enable_interrupt();
    systick::set_rvr(1 << 23);
    systick::clear_cvr();

    loop {}
}
