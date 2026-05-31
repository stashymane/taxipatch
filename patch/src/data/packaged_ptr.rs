use std::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
pub struct PackagedPtr<T> {
    addr: usize,
    _marker: PhantomData<fn() -> T>,
}

impl<T> PackagedPtr<T> {
    pub const fn new(addr: usize) -> Self {
        Self {
            addr,
            _marker: PhantomData,
        }
    }

    pub const fn addr(self) -> usize {
        self.addr
    }

    pub const fn ptr(self) -> *mut T {
        self.addr as *mut T
    }

    pub unsafe fn read(self) -> T
    where
        T: Copy,
    {
        unsafe { *self.ptr() }
    }

    pub unsafe fn write(self, value: T) {
        unsafe {
            *self.ptr() = value;
        }
    }
}
