use core::marker::PhantomData;

use crate::{pin::Pin, port_reg::PortReg};

pub trait PortID {
    const BASE_ADDR: u32;
}

pub struct Port<'a, T: PortID> {
    port_reg: PortReg<'a>,
    pins: Pins<'a, T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: PortID> Port<'a, T> {
    const NUM_OF_PIN: u8 = 32;
    pub fn new() -> Self {
        Self {
            port_reg: PortReg::new(T::BASE_ADDR),
            pins: Pins::new(),
            _marker: PhantomData,
        }
    }
}

struct Pins<'a, T: PortID> {
    pin00: Pin<'a, T, 0>,
    pin01: Pin<'a, T, 1>,
    pin02: Pin<'a, T, 2>,
    pin03: Pin<'a, T, 3>,
    pin04: Pin<'a, T, 4>,
    pin05: Pin<'a, T, 5>,
    pin06: Pin<'a, T, 6>,
    pin07: Pin<'a, T, 7>,
    pin08: Pin<'a, T, 8>,
    pin09: Pin<'a, T, 9>,
    pin10: Pin<'a, T, 10>,
    pin11: Pin<'a, T, 11>,
    pin12: Pin<'a, T, 12>,
    pin13: Pin<'a, T, 13>,
    pin14: Pin<'a, T, 14>,
    pin15: Pin<'a, T, 15>,
    pin16: Pin<'a, T, 16>,
    pin17: Pin<'a, T, 17>,
    pin18: Pin<'a, T, 18>,
    pin19: Pin<'a, T, 19>,
    pin20: Pin<'a, T, 20>,
    pin21: Pin<'a, T, 21>,
    pin22: Pin<'a, T, 22>,
    pin23: Pin<'a, T, 23>,
    pin24: Pin<'a, T, 24>,
    pin25: Pin<'a, T, 25>,
    pin26: Pin<'a, T, 26>,
    pin27: Pin<'a, T, 27>,
    pin28: Pin<'a, T, 28>,
    pin29: Pin<'a, T, 29>,
    pin30: Pin<'a, T, 30>,
    pin31: Pin<'a, T, 31>,
}

impl<'a, T: PortID> Pins<'a, T> {
    pub fn new() -> Self {
        Self {
            pin00: Pin::<'a, T, 0>::new(),
            pin01: Pin::<'a, T, 1>::new(),
            pin02: Pin::<'a, T, 2>::new(),
            pin03: Pin::<'a, T, 3>::new(),
            pin04: Pin::<'a, T, 4>::new(),
            pin05: Pin::<'a, T, 5>::new(),
            pin06: Pin::<'a, T, 6>::new(),
            pin07: Pin::<'a, T, 7>::new(),
            pin08: Pin::<'a, T, 8>::new(),
            pin09: Pin::<'a, T, 9>::new(),
            pin10: Pin::<'a, T, 10>::new(),
            pin11: Pin::<'a, T, 11>::new(),
            pin12: Pin::<'a, T, 12>::new(),
            pin13: Pin::<'a, T, 13>::new(),
            pin14: Pin::<'a, T, 14>::new(),
            pin15: Pin::<'a, T, 15>::new(),
            pin16: Pin::<'a, T, 16>::new(),
            pin17: Pin::<'a, T, 17>::new(),
            pin18: Pin::<'a, T, 18>::new(),
            pin19: Pin::<'a, T, 19>::new(),
            pin20: Pin::<'a, T, 20>::new(),
            pin21: Pin::<'a, T, 21>::new(),
            pin22: Pin::<'a, T, 22>::new(),
            pin23: Pin::<'a, T, 23>::new(),
            pin24: Pin::<'a, T, 24>::new(),
            pin25: Pin::<'a, T, 25>::new(),
            pin26: Pin::<'a, T, 26>::new(),
            pin27: Pin::<'a, T, 27>::new(),
            pin28: Pin::<'a, T, 28>::new(),
            pin29: Pin::<'a, T, 29>::new(),
            pin30: Pin::<'a, T, 30>::new(),
            pin31: Pin::<'a, T, 31>::new(),
        }
    }
}
