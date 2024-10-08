use core::{
    cell::UnsafeCell,
    ptr::{read_volatile, write_volatile},
};

#[repr(transparent)]
pub struct RO<T>
where
    T: Copy,
{
    addr: UnsafeCell<T>,
}

impl<T> RO<T>
where
    T: Copy,
{
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe { read_volatile(self.addr.get()) }
    }
}

#[repr(transparent)]
pub struct RW<T>
where
    T: Copy,
{
    addr: UnsafeCell<T>,
}

impl<T> RW<T>
where
    T: Copy,
{
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe { read_volatile(self.addr.get()) }
    }

    #[inline(always)]
    pub fn write(&self, val: T) {
        unsafe {
            write_volatile(self.addr.get(), val);
        }
    }
}
