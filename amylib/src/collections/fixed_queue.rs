use std::{mem::MaybeUninit, ops::{Index, IndexMut}};

// UNDER CONSTRUCTION

pub struct FixedDeque<T, const N: usize> {
    head: usize,
    len: usize,
    buf: [std::mem::MaybeUninit<T>; N],
}

impl<T, const N: usize> Drop for FixedDeque<T, N> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

impl<T, const N: usize> Default for FixedDeque<T, N> {
    fn default() -> Self {
        Self {
            head: 0,
            len: 0,
            buf: unsafe { MaybeUninit::uninit().assume_init() }
        }
    }
}

impl<T, const N: usize> FixedDeque<T, N> {
    pub const fn new() -> Self {
        Self {
            head: 0,
            len: 0,
            buf: unsafe { MaybeUninit::uninit().assume_init() },
        }
    }

    #[inline]
    fn wrapped_idx(&self, index: usize) -> usize {
        let idx = self.head.wrapping_add(index);
        debug_assert!(idx < N || (idx - N) < N, "idx: {idx}, N: {N}");
        if idx >= N { idx - N } else { idx }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.len == N
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(unsafe { self.buf[self.wrapped_idx(index)].assume_init_ref() })
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            Some(unsafe { self.buf[self.wrapped_idx(index)].assume_init_mut() })
        } else {
            None
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub const fn capacity(&self) -> usize {
        N
    }

    pub fn push_front(&mut self, value: T) {
        let n = self.head.wrapping_sub(1).wrapping_add(N);
        println!("{} - 1 % {N} = {n}", self.head);
        self.head = self.wrapped_idx(n);
        self.buf[self.head].write(value);
        if self.len < N { self.len += 1; }
    }

    pub fn push_back(&mut self, value: T) {
        self.buf[self.wrapped_idx(self.len)].write(value);
        if self.len == N { self.head = self.wrapped_idx(1); }
        if self.len < N { self.len += 1; }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let old_head = self.head;
            self.head = self.wrapped_idx(1);
            self.len -= 1;
            Some(unsafe { std::mem::replace(&mut self.buf[old_head], MaybeUninit::uninit()).assume_init() })
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.len -= 1;
            Some(unsafe { std::mem::replace(&mut self.buf[self.wrapped_idx(self.len)], MaybeUninit::uninit()).assume_init() })
        }
    }

    pub fn front(&self) -> Option<&T> {
        if self.len > 0 {
            return Some(unsafe { self.buf[self.head].assume_init_ref() });
        }
        None
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        if self.len > 0 {
            return Some(unsafe { self.buf[self.head].assume_init_mut() });
        }
        None
    }

    pub fn back(&self) -> Option<&T> {
        self.len.checked_sub(1).map(|idx| unsafe { self.buf[self.wrapped_idx(idx)].assume_init_ref() })
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.len.checked_sub(1).map(|idx| unsafe { self.buf[self.wrapped_idx(idx)].assume_init_mut() })
    }

    #[inline]
    pub fn as_slices(&self) -> (&[T], &[T]) {
        let split_idx = (self.head + self.len).min(N);
        unsafe {(
            &*(&self.buf[self.head..split_idx]           as *const [MaybeUninit<T>] as *const [T]),
            &*(&self.buf[0..split_idx.saturating_sub(N)] as *const [MaybeUninit<T>] as *const [T]),
        )}
    }

    #[inline]
    pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
        let split_idx = (self.head + self.len).min(N);
        unsafe {(
            &mut *(&mut self.buf[self.head..split_idx]           as *mut [MaybeUninit<T>] as *mut [T]),
            &mut *(&mut self.buf[0..split_idx.saturating_sub(N)] as *mut [MaybeUninit<T>] as *mut [T]),
        )}
    }
}

impl<T, const N: usize> Index<usize> for FixedDeque<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len);
        unsafe { self.buf[self.wrapped_idx(index)].assume_init_ref() }
    }
}

impl<T, const N: usize> IndexMut<usize> for FixedDeque<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.len);
        unsafe { self.buf[self.wrapped_idx(index)].assume_init_mut() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        const N: usize = 10;
        let mut q = FixedDeque::<usize, N>::new();
        for i in 0..2*N {
            assert_eq!(q.wrapped_idx(i), i % N);
        }
        assert_eq!(q.wrapped_idx(q.head.wrapping_sub(1).wrapping_add(N)), N - 1);
        q.push_front(5);
        for i in 0..2*N {
            assert_eq!(q.wrapped_idx(i), (N + i - 1) % N);
        }
        assert_eq!(q.wrapped_idx(q.head.wrapping_sub(1).wrapping_add(N)), N - 1);
    }

    #[test]
    fn test1() {
        let mut q = FixedDeque::<usize, 3>::new();
        q.push_back(7);
        assert_eq!(q.back(), Some(&7));
    }

    #[test]
    fn test2() {
        let mut q = FixedDeque::<usize, 3>::new();
        q.push_back(7);
        assert_eq!(q.back(), Some(&7));
        q.push_back(4);
        assert_eq!(q.back(), Some(&4));
    }

    #[test]
    fn test3() {
        let mut q = FixedDeque::<usize, 3>::new();
        q.push_back(7);
        assert_eq!(q.back(), Some(&7));
        q.push_front(4);
        assert_eq!(q.front(), Some(&4));
        assert_eq!(q.back(), Some(&7));
        q.push_back(8);
        assert_eq!(q.front(), Some(&4));
        assert_eq!(q.back(), Some(&8));
    }

    #[test]
    fn test_wrap_back() {
        let mut q = FixedDeque::<usize, 3>::new();
        q.push_back(1);
        assert_eq!(q.back(), Some(&1));
        assert_eq!(q.front(), Some(&1));
        q.push_back(2);
        assert_eq!(q.back(), Some(&2));
        assert_eq!(q.front(), Some(&1));
        q.push_back(3);
        assert_eq!(q.back(), Some(&3));
        assert_eq!(q.front(), Some(&1));
        q.push_back(4);
        assert_eq!(q.back(), Some(&4));
        assert_eq!(q.front(), Some(&2));
        assert_eq!([q[0], q[1], q[2]], [2, 3, 4]);
    }

    #[test]
    fn test_wrap_front() {
        let mut q = FixedDeque::<usize, 3>::new();
        q.push_front(1);
        assert_eq!(q.front(), Some(&1));
        assert_eq!(q.back(), Some(&1));
        q.push_front(2);
        assert_eq!(q.front(), Some(&2));
        assert_eq!(q.back(), Some(&1));
        q.push_front(3);
        assert_eq!(q.front(), Some(&3));
        assert_eq!(q.back(), Some(&1));
        q.push_front(4);
        assert_eq!(q.front(), Some(&4));
        assert_eq!(q.back(), Some(&2));
        assert_eq!([q[0], q[1], q[2]], [4, 3, 2]);
    }
}
