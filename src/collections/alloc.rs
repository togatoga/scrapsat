use std::{
    alloc::{alloc, dealloc, realloc, Layout},
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut},
    ptr::NonNull,
};

/// `Id` is a offset from a `ptr`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id<T>(pub u32, PhantomData<fn(T) -> T>);
impl<T> std::ops::Add<usize> for Id<T> {
    type Output = Self;
    fn add(self, rhs: usize) -> Self {
        Id(self.0 + rhs as u32, PhantomData)
    }
}

impl<T> Id<T> {
    pub const UNDEF: Id<T> = Id(std::u32::MAX, PhantomData);
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        Self::UNDEF
    }
}

#[derive(Debug)]
pub struct RegionAllocator<T: Clone + Copy> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
    // for internal
    align: usize,
    elem_size: usize,
}

impl<T: Clone + Copy> RegionAllocator<T> {
    pub fn new() -> RegionAllocator<T> {
        RegionAllocator {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
            align: std::mem::align_of::<T>(),
            elem_size: std::mem::size_of::<T>(),
        }
    }
    pub fn with_capacity(n: usize) -> RegionAllocator<T> {
        let mut ra = RegionAllocator::new();
        ra.grow(n);
        ra
    }
    pub fn grow(&mut self, additional: usize) {
        if self.len() + additional <= self.capacity() {
            return;
        }
        let mut new_cap = std::cmp::max(1, self.cap);
        while new_cap < self.len() + additional {
            new_cap <<= 1;
        }
        unsafe {
            let ptr = if self.cap == 0 {
                let layout =
                    Layout::from_size_align_unchecked(new_cap * self.elem_size, self.align);
                alloc(layout)
            } else {
                let old_layout =
                    Layout::from_size_align_unchecked(self.cap * self.elem_size, self.align);

                realloc(
                    self.ptr.as_ptr() as *mut u8,
                    old_layout,
                    new_cap * self.elem_size,
                )
            };
            debug_assert!(!ptr.is_null(), "Out of the memory");

            self.ptr = NonNull::new_unchecked(ptr as *mut T);
            self.cap = new_cap;
        }
    }

    pub fn alloc(&mut self, elem: T) -> Id<T> {
        if self.len() >= self.capacity() {
            self.grow(1);
        }

        unsafe {
            let ptr_last = self.ptr.as_ptr().add(self.len);
            std::ptr::write(ptr_last, elem);
            self.len += 1;
            Id(self.len() as u32 - 1, PhantomData)
        }
    }

    pub fn reset(&mut self) {
        self.len = 0;
    }

    // Getter functions
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn subslice(&self, idx: Id<T>, len: usize) -> &[T] {
        let id = idx.0 as usize;
        &self.deref()[id..id + len]
    }

    pub fn subslice_mut(&mut self, idx: Id<T>, len: usize) -> &mut [T] {
        let id = idx.0 as usize;
        &mut self.deref_mut()[id..id + len]
    }

    pub fn get(&self, idx: Id<T>) -> &T {
        &self[idx.0 as usize]
    }
    pub fn get_mut(&mut self, idx: Id<T>) -> &mut T {
        &mut self[idx.0 as usize]
    }
}

impl<T: Clone + Copy> Default for RegionAllocator<T> {
    fn default() -> Self {
        RegionAllocator::with_capacity(1024)
    }
}

impl<T: Clone + Copy + Debug> Index<Id<T>> for RegionAllocator<T> {
    type Output = T;
    fn index(&self, idx: Id<T>) -> &Self::Output {
        &self.deref()[idx.0 as usize]
    }
}

impl<T: Clone + Copy + Debug> IndexMut<Id<T>> for RegionAllocator<T> {
    fn index_mut(&mut self, idx: Id<T>) -> &mut Self::Output {
        &mut self.deref_mut()[idx.0 as usize]
    }
}

impl<T: Clone + Copy> Deref for RegionAllocator<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len()) }
    }
}

impl<T: Clone + Copy> DerefMut for RegionAllocator<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len()) }
    }
}

impl<T: Clone + Copy> Drop for RegionAllocator<T> {
    fn drop(&mut self) {
        if self.capacity() != 0 {
            unsafe {
                let layout =
                    Layout::from_size_align_unchecked(self.cap * self.elem_size, self.align);
                dealloc(self.ptr.as_ptr() as *mut _, layout);
            }
        }
    }
}
