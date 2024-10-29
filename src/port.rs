use core::ptr::addr_of;

pub struct PortGroup {}

impl PortGroup {
    const PORT_BASE: u32 = 0x40008000;

    pub fn get_port(offset: u32) -> Port {
        Port::new(Self::PORT_BASE + 0x80 * offset)
    }
}

pub struct Port {
    base: *const u32,
}

impl Port {
    fn new(base: u32) -> Self {
        Self {
            base: addr_of!(base),
        }
    }
}
