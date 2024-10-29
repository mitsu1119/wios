use crate::port::Port;

static mut PERIPHERAL_USED: bool = false;

#[allow(non_snake_case)]
pub struct Peripheral {
    PA: Port,
    PB: Port,
    PC: Port,
    PD: Port,
}

impl Peripheral {
    fn get_port(offset: u32) -> Port {
        const PORT_BASE: u32 = 0x40008000;
        Port::new(PORT_BASE + 0x80 * offset)
    }

    pub fn take() -> Option<Self> {
        if unsafe { PERIPHERAL_USED } {
            None
        } else {
            unsafe { PERIPHERAL_USED = true };
            Some(Self {
                PA: Self::get_port(0),
                PB: Self::get_port(1),
                PC: Self::get_port(2),
                PD: Self::get_port(3),
            })
        }
    }
}
