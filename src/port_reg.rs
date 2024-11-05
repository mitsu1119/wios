use crate::register::{RO, RW};

pub struct PortReg<'a> {
    dir: RW<'a, u32>,
    dirclr: RW<'a, u32>,
    dirset: RW<'a, u32>,
    dirtgl: RW<'a, u32>,
    out: RW<'a, u32>,
    outclr: RW<'a, u32>,
    outset: RW<'a, u32>,
    outtgl: RW<'a, u32>,
    r#in: RO<'a, u32>,
    // ctrl: RW<'a, u32>,
    // wrconfig: WO<'a, u32>,
    // evctrl: RW<'a, u32>,
    // pmux: [RW<'a, u8>; 16],
    // pincfg: [RW<'a, u8>; 32],
}

impl<'a> PortReg<'a> {
    pub fn new(base_addr: u32) -> Self {
        Self {
            dir: RW::new((base_addr + 0x00) as *mut u32),
            dirclr: RW::new((base_addr + 0x04) as *mut u32),
            dirset: RW::new((base_addr + 0x08) as *mut u32),
            dirtgl: RW::new((base_addr + 0x0c) as *mut u32),
            out: RW::new((base_addr + 0x10) as *mut u32),
            outclr: RW::new((base_addr + 0x14) as *mut u32),
            outset: RW::new((base_addr + 0x18) as *mut u32),
            outtgl: RW::new((base_addr + 0x1c) as *mut u32),
            r#in: RO::new((base_addr + 0x20) as *mut u32),
        }
    }
}
