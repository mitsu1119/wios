use core::{
    cell::UnsafeCell,
    ptr::{read_volatile, write_volatile},
};

#[repr(transparent)]
pub struct RO<'a, T>
where
    T: Copy,
{
    addr: &'a mut UnsafeCell<T>,
}

impl<'a, T> RO<'a, T>
where
    T: Copy,
{
    pub fn new(addr: *mut T) -> Self {
        Self {
            addr: unsafe { UnsafeCell::from_mut(&mut *addr) },
        }
    }

    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe { read_volatile(self.addr.get()) }
    }
}

#[repr(transparent)]
pub struct WO<'a, T>
where
    T: Copy,
{
    addr: &'a UnsafeCell<T>,
}

impl<'a, T> WO<'a, T>
where
    T: Copy,
{
    pub fn new(addr: *mut T) -> Self {
        Self {
            addr: unsafe { UnsafeCell::from_mut(&mut *addr) },
        }
    }

    #[inline(always)]
    pub fn write(&self, val: T) {
        unsafe {
            write_volatile(self.addr.get(), val);
        }
    }
}

#[repr(transparent)]
pub struct RW<'a, T>
where
    T: Copy,
{
    addr: &'a UnsafeCell<T>,
}

impl<'a, T> RW<'a, T>
where
    T: Copy,
{
    pub fn new(addr: *mut T) -> Self {
        Self {
            addr: unsafe { UnsafeCell::from_mut(&mut *addr) },
        }
    }

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
