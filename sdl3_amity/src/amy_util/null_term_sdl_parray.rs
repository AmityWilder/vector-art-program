use std::{cell::OnceCell, marker::PhantomData, ptr::NonNull};

pub struct NullTermPIter<'a, T: 'a> {
    ptr: NonNull<*mut T>,
    _lt: PhantomData<&'a NullTermSdlPArray<T>>,
}

impl<'a, T: 'a> Iterator for NullTermPIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = unsafe { self.ptr.read().as_ref::<'a>() };
        self.ptr = unsafe { self.ptr.add(1) };
        curr
    }
}

pub struct NullTermPIterMut<'a, T: 'a> {
    ptr: NonNull<*mut T>,
    _lt: PhantomData<&'a mut NullTermSdlPArray<T>>,
}

impl<'a, T: 'a> Iterator for NullTermPIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = unsafe { self.ptr.read().as_mut::<'a>() };
        self.ptr = unsafe { self.ptr.add(1) };
        curr
    }
}

/// Null-terminated pointer array
pub struct NullTermSdlPArray<T> {
    ptr: NonNull<*mut T>,
    len: OnceCell<usize>,
}

impl<T> Drop for NullTermSdlPArray<T> {
    fn drop(&mut self) {
        unsafe { sdl3_sys::stdinc::SDL_free(self.ptr.as_ptr().cast()) }
    }
}

impl<T> NullTermSdlPArray<T> {
    pub(crate) fn new(ptr: *mut *mut T) -> Option<Self> {
        if let Some(ptr) = NonNull::new(unsafe { &mut *ptr.cast() }) {
            Some(Self {
                ptr,
                len: OnceCell::new(),
            })
        } else { None }
    }

    /// Null-terminated iterator
    #[inline]
    pub fn iter(&self) -> NullTermPIter<'_, T> {
        NullTermPIter {
            ptr: self.ptr.clone(),
            _lt: PhantomData,
        }
    }

    /// Null-terminated iterator
    #[inline]
    pub fn iter_mut(&mut self) -> NullTermPIterMut<'_, T> {
        NullTermPIterMut {
            ptr: self.ptr.clone(),
            _lt: PhantomData,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        *self.len.get_or_init(|| self.iter().count())
    }

    #[inline]
    pub fn as_slice(&self) -> &[&T] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr().cast(), self.len()) }
    }

    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [&mut T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr().cast(), self.len()) }
    }
}
