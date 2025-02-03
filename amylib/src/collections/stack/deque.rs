use std::collections::VecDeque;
use super::{Stack, SizedStack};

pub struct VecDestack<T>(VecDeque<T>);

impl<T> FromIterator<T> for VecDestack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(VecDeque::from_iter(iter))
    }
}

impl<T, V: Into<VecDeque<T>>> From<V> for VecDestack<T> {
    fn from(value: V) -> Self {
        Self(value.into())
    }
}

impl<T> VecDestack<T> {
    /// push an item to the top of the stack.
    /// if the stack is full, eject the oldest item.
    pub fn push_no_resize(&mut self, value: T) -> Option<T> {
        let ejected = if self.is_full() { self.0.pop_front() } else { None };
        self.push(value);
        ejected
    }
}

impl<T> Default for VecDestack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> VecDestack<T> {
    pub const fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(VecDeque::with_capacity(capacity))
    }

    /// `len == capacity`
    pub fn is_full(&self) -> bool {
        self.0.len() == self.0.capacity()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.0.push_back(value);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop_back()
    }

    #[inline]
    pub fn top(&mut self) -> Option<&T> {
        self.0.back()
    }
    #[inline]
    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.0.back_mut()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T> Stack<T> for VecDestack<T> {
    #[inline] fn new() -> Self { Self::new() }
    #[inline] fn push(&mut self, value: T) { self.push(value); }
    #[inline] fn pop(&mut self) -> Option<T> { self.pop() }
    #[inline] fn top(&mut self) -> Option<&T> { self.top() }
    #[inline] fn top_mut(&mut self) -> Option<&mut T> { self.top_mut() }
    #[inline] fn clear(&mut self) { self.clear(); }
    #[inline] fn is_empty(&self) -> bool { self.is_empty() }
}

impl<T> SizedStack<T> for VecDestack<T> {
    #[inline] fn with_capacity(capacity: usize) -> Self { Self::with_capacity(capacity) }
    #[inline] fn is_full(&self) -> bool { self.is_full() }
    #[inline] fn len(&self) -> usize { self.len() }
    #[inline] fn reserve(&mut self, additional: usize) { self.reserve(additional) }
}
