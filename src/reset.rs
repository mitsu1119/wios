use core::ptr::{self, addr_of, addr_of_mut};

use cortex_m_semihosting::hprintln;

use crate::{
    linked_list::LinkedListItem,
    peripheral::Peripheral,
    process::{self, Process, PS_STACK, PS_STACK2},
    scheduler::Scheduler,
    systick,
};

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

    systick_init();

    let mut ps1 = LinkedListItem::new(Process::new(&PS_STACK, process::process_main1));
    let mut ps2 = LinkedListItem::new(Process::new(&PS_STACK2, process::process_main2));
    let mut scheduler = Scheduler::new();
    scheduler.register(&mut ps1);
    scheduler.register(&mut ps2);

    scheduler.exec();

    let peripheral = Peripheral::take().unwrap();

    /*
    Pin<PinID> { port_regs: PortReg }

    Port<PortGroupNum> {
        Pin<0>,
        Pin<1>,
        ...
    }

    LED {
        new(peripheral) {
            init(peripheral.PA.pin15)
        }
    }

    let peripheral = Peripheral::new();
    let pa_pins = peripheral.PA;
    let led = pa_pins.led;
    */

    loop {}
}

fn systick_init() {
    // SysTick 初期化
    hprintln!("SysTick init");
    systick::enable();
    systick::enable_interrupt();
    systick::set_rvr(1 << 23);
    systick::clear_cvr();
}
