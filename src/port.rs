use core::ptr::addr_of;

pub struct Port {
    base: *const u32,
}

impl Port {
    pub fn new(base: u32) -> Self {
        Self {
            base: addr_of!(base),
        }
    }
}
