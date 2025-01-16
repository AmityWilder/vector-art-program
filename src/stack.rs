use std::collections::LinkedList;

pub trait StackAdaptor<T> {
    fn new() -> Self;
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn top(&mut self) -> Option<&T>;
    fn top_mut(&mut self) -> Option<&mut T>;
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

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub struct VecStack<T>(Vec<T>);

impl<T> StackAdaptor<T> for VecStack<T> {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, value: T) {
        self.0.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    fn top(&mut self) -> Option<&T> {
        self.0.last()
    }
    fn top_mut(&mut self) -> Option<&mut T> {
        self.0.last_mut()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
