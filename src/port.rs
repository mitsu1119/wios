use crate::port_reg::PortReg;

pub struct Port<'a> {
    base: *const u32,
    port_reg: PortReg<'a>,
}

impl<'a> Port<'a> {
    pub fn new(base: *const u32) -> Self {
        Self {
            base,
            port_reg: PortReg::new(base as u32),
        }
    }
}
