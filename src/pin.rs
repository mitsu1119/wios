use core::marker::PhantomData;

use crate::port::PortID;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Pin<'a, T: PortID, const PIN_ID: u32> {
    _marker: PhantomData<&'a T>,
}

impl<'a, T: PortID, const PIN_ID: u32> Pin<'a, T, PIN_ID> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
