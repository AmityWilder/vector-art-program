pub trait Stack<T> {
    fn new() -> Self;
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn top(&mut self) -> Option<&T>;
    fn top_mut(&mut self) -> Option<&mut T>;
    fn clear(&mut self);
    fn is_empty(&self) -> bool;
}

pub trait SizedStack<T>: Stack<T> {
    fn with_capacity(capacity: usize) -> Self;
    fn is_full(&self) -> bool;
    fn len(&self) -> usize;
    fn reserve(&mut self, additional: usize);
}

pub mod linked;
pub mod vec;
pub mod deque;
