use std::ops::{Index, IndexMut};

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct RawArray<T> {
    ptr: *mut T,
}

impl<T> RawArray<T> {
    pub fn as_ptr(&self) -> *mut T {
        self.ptr
    }

    pub unsafe fn get(&self, index: usize) -> &T {
        unsafe { &*self.ptr.add(index) }
    }

    pub unsafe fn get_mut(&mut self, index: usize) -> &mut T {
        unsafe { &mut *self.ptr.add(index) }
    }
}

impl<T> Index<usize> for RawArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.get(index) }
    }
}

impl<T> IndexMut<usize> for RawArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { self.get_mut(index) }
    }
}
