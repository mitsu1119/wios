use core::marker::PhantomData;

use crate::port_reg::PortReg;

pub trait PortID {
    const BASE_ADDR: u32;
}

pub struct Port<'a, T: PortID> {
    port_reg: PortReg<'a>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: PortID> Port<'a, T> {
    pub fn new() -> Self {
        Self {
            port_reg: PortReg::new(T::BASE_ADDR),
            _marker: PhantomData,
        }
    }
}
