use crate::reset;

// vector table のエントリ
pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

impl Vector {
    const fn new(handler: unsafe extern "C" fn()) -> Self {
        Self { handler }
    }

    const fn reserved() -> Self {
        Self { reserved: 0 }
    }
}

// デフォルトの例外ハンドラ
#[no_mangle]
pub extern "C" fn default_exception_handler() {
    let _x = 10;
    loop {}
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 15] = [
    Vector::new(reset::reset),              // 1. Reset
    Vector::new(default_exception_handler), // 2. NMI
    Vector::new(default_exception_handler), // 3. HardFault
    Vector::new(default_exception_handler), // 4. MemManage
    Vector::new(default_exception_handler), // 5. BusFault
    Vector::new(default_exception_handler), // 6. UsageFault
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::new(default_exception_handler), // 11. SVCall
    Vector::new(default_exception_handler), // 12. DebugMonitor
    Vector::reserved(),
    Vector::new(default_exception_handler), // 14. PendSV
    Vector::new(default_exception_handler), // 15. SysTick
];
