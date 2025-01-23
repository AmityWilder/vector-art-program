use std::collections::{LinkedList, VecDeque};

pub trait StackAdaptor<T> {
    fn new() -> Self;
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn top(&mut self) -> Option<&T>;
    fn top_mut(&mut self) -> Option<&mut T>;
    fn clear(&mut self);
    fn is_empty(&self) -> bool;
}

pub struct ListStack<T>(LinkedList<T>);

impl<T> StackAdaptor<T> for ListStack<T> {
    fn new() -> Self {
        Self(LinkedList::new())
    }

    fn push(&mut self, value: T) {
        self.0.push_front(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.0.pop_front()
    }

    fn top(&mut self) -> Option<&T> {
        self.0.front()
    }
    fn top_mut(&mut self) -> Option<&mut T> {
        self.0.front_mut()
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub struct VecStack<T>(VecDeque<T>);

impl<T> VecStack<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(VecDeque::with_capacity(capacity))
    }

    /// `len == capacity`
    pub fn is_full(&self) -> bool {
        self.0.len() == self.0.capacity()
    }

    /// push an item to the top of the stack.
    /// if the stack is full, eject the oldest item.
    pub fn push_no_resize(&mut self, value: T) -> Option<T> {
        let ejected = if self.is_full() { self.0.pop_front() } else { None };
        self.push(value);
        ejected
    }
}

impl<T> StackAdaptor<T> for VecStack<T> {
    fn new() -> Self {
        Self(VecDeque::new())
    }

    fn push(&mut self, value: T) {
        self.0.push_back(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.0.pop_back()
    }

    fn top(&mut self) -> Option<&T> {
        self.0.back()
    }
    fn top_mut(&mut self) -> Option<&mut T> {
        self.0.back_mut()
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
