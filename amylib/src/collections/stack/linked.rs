use std::collections::LinkedList;
use super::Stack;

pub struct LinkedStack<T>(LinkedList<T>);

impl<T> FromIterator<T> for LinkedStack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = LinkedList::new();
        for item in iter {
            list.push_front(item);
        }
        Self(list)
    }
}

impl<T> LinkedStack<T> {
    pub const fn new() -> Self {
        Self(LinkedList::new())
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.0.push_front(value);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop_front()
    }

    #[inline]
    pub fn top(&mut self) -> Option<&T> {
        self.0.front()
    }
    #[inline]
    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.0.front_mut()
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

impl<T> Stack<T> for LinkedStack<T> {
    #[inline] fn new() -> Self { Self::new() }
    #[inline] fn push(&mut self, value: T) { self.push(value); }
    #[inline] fn pop(&mut self) -> Option<T> { self.pop() }
    #[inline] fn top(&mut self) -> Option<&T> { self.top() }
    #[inline] fn top_mut(&mut self) -> Option<&mut T> { self.top_mut() }
    #[inline] fn clear(&mut self) { self.clear(); }
    #[inline] fn is_empty(&self) -> bool { self.is_empty() }
}
