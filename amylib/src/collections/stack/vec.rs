use super::{Stack, SizedStack};

pub struct VecStack<T>(Vec<T>);

impl<T> FromIterator<T> for VecStack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl<T, V: Into<Vec<T>>> From<V> for VecStack<T> {
    fn from(value: V) -> Self {
        Self(value.into())
    }
}

impl<T> VecStack<T> {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

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
        self.0.push(value);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    #[inline]
    pub fn top(&mut self) -> Option<&T> {
        self.0.last()
    }
    #[inline]
    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.0.last_mut()
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

impl<T> Stack<T> for VecStack<T> {
    #[inline] fn new() -> Self { Self::new() }
    #[inline] fn push(&mut self, value: T) { self.push(value); }
    #[inline] fn pop(&mut self) -> Option<T> { self.pop() }
    #[inline] fn top(&mut self) -> Option<&T> { self.top() }
    #[inline] fn top_mut(&mut self) -> Option<&mut T> { self.top_mut() }
    #[inline] fn clear(&mut self) { self.clear(); }
    #[inline] fn is_empty(&self) -> bool { self.is_empty() }
}

impl<T> SizedStack<T> for VecStack<T> {
    #[inline] fn with_capacity(capacity: usize) -> Self { Self::with_capacity(capacity) }
    #[inline] fn is_full(&self) -> bool { self.is_full() }
    #[inline] fn len(&self) -> usize { self.len() }
    #[inline] fn reserve(&mut self, additional: usize) { self.reserve(additional) }
}
