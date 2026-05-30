use std::marker::PhantomData;

#[derive(Debug)]
pub struct Pointers {
    pub dw_creation_width: PackagedPtr<u32>,
    pub dw_creation_height: PackagedPtr<u32>,

    pub game_stage: PackagedPtr<u32>,
    pub game_substage: PackagedPtr<u32>,
}

impl Pointers {
    pub fn from(base: usize) -> Self {
        Self {
            dw_creation_width: PackagedPtr::new(base + 0x001EC5F8),
            dw_creation_height: PackagedPtr::new(base + 0x001EC5FC),

            game_stage: PackagedPtr::new(base + 0x003bc330),
            game_substage: PackagedPtr::new(base + 0x003bc334),
        }
    }
}

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
