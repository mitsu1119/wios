use crate::port::{Port, PortID};

static mut PERIPHERAL_USED: bool = false;

pub struct Peripheral<'a> {
    pub pa: Port<'a, PA>,
    pub pb: Port<'a, PB>,
    pub pc: Port<'a, PC>,
    pub pd: Port<'a, PD>,
}

impl<'a> Peripheral<'a> {
    pub fn take() -> Option<Self> {
        if unsafe { PERIPHERAL_USED } {
            None
        } else {
            unsafe { PERIPHERAL_USED = true };
            Some(Self {
                pa: Port::<'a, PA>::new(),
                pb: Port::<'a, PB>::new(),
                pc: Port::<'a, PC>::new(),
                pd: Port::<'a, PD>::new(),
            })
        }
    }
}

pub struct PA {}
impl PortID for PA {
    const BASE_ADDR: u32 = 0x40008000;
}

pub struct PB {}
impl PortID for PB {
    const BASE_ADDR: u32 = 0x40008000 + 0x80;
}

pub struct PC {}
impl PortID for PC {
    const BASE_ADDR: u32 = 0x40008000 + 0x80 * 2;
}

pub struct PD {}
impl PortID for PD {
    const BASE_ADDR: u32 = 0x40008000 + 0x80 * 3;
}
