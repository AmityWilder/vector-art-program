use std::collections::VecDeque;
use crate::rc::{Owned, Strong, StrongMut};
use super::stack::VecStack;

pub trait Recursive: Sized {
    type Node;
    fn get_if_node(&self) -> Option<&Self::Node>;
    fn get_if_node_mut(&mut self) -> Option<&mut Self::Node>;
    fn children(node: &Self::Node) -> &Tree<Self>;
    fn children_mut(node: &mut Self::Node) -> &mut Tree<Self>;
}

pub struct Iter<'a, T> {
    iter: std::slice::Iter<'a, StrongMut<T>>,
}

impl<'a, T: Recursive> Iter<'a, T> {
    fn new(src: &'a Tree<T>) -> Self {
        Self { iter: src.0.iter() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = Strong<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|rc| rc.clone())
    }
}
impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|rc| rc.clone())
    }
}
impl<'a, T> ExactSizeIterator for Iter<'a, T> {}

pub struct IterMut<'a, T> {
    iter: std::slice::IterMut<'a, StrongMut<T>>,
}

impl<'a, T: Recursive> IterMut<'a, T> {
    fn new(src: &'a mut Tree<T>) -> Self {
        Self { iter: src.0.iter_mut() }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = StrongMut<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|rc| rc.clone_mut())
    }
}
impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|rc| rc.clone_mut())
    }
}
impl<'a, T> ExactSizeIterator for IterMut<'a, T> {}

pub struct Tree<T: Recursive>(Vec<StrongMut<T>>);

impl<T: Recursive, V: Into<Vec<StrongMut<T>>>> From<V> for Tree<T> {
    fn from(value: V) -> Self {
        Self(value.into())
    }
}

impl<T: Recursive> FromIterator<StrongMut<T>> for Tree<T> {
    fn from_iter<I: IntoIterator<Item = StrongMut<T>>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<T: Recursive> FromIterator<T> for Tree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().map(|x| StrongMut::new(x)).collect())
    }
}

impl<T: Recursive> std::ops::Index<usize> for Tree<T> {
    type Output = StrongMut<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Recursive> std::ops::IndexMut<usize> for Tree<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Recursive> Tree<T> {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.0.len() == self.0.capacity()
    }

    #[inline]
    pub fn push(&mut self, value: StrongMut<T>) {
        self.0.push(value)
    }

    #[inline]
    pub fn pop(&mut self) -> Option<StrongMut<T>> {
        self.0.pop()
    }

    #[inline]
    pub fn insert(&mut self, index: usize, element: StrongMut<T>) {
        self.0.insert(index, element)
    }

    #[inline]
    pub fn remove(&mut self, index: usize) -> StrongMut<T> {
        self.0.remove(index)
    }

    /// Iterate over only the topmost layer
    #[inline]
    pub fn shallow_iter<'a>(&'a self) -> Iter<'a, T> {
        Iter::new(self)
    }

    /// Iterate over only the topmost layer
    #[inline]
    pub fn shallow_iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut::new(self)
    }

    #[inline]
    pub fn dfs_iter<P: Fn(&T::Node) -> bool>(&self, delve: P) -> DepthFirstIter<T, P> {
        DepthFirstIter::new(self.0.iter(), delve)
    }

    #[inline]
    pub fn dfs_iter_mut<P: Fn(&T::Node) -> bool>(&mut self, delve: P) -> DepthFirstIterMut<T, P> {
        DepthFirstIterMut::new(self.0.iter(), delve)
    }
}

pub trait RecursiveIterator: Iterator<Item: Owned<Inner = Self::Inner>> {
    type Inner: Recursive;

    /// Depth of the most recent item to have been returned by `next()`
    fn last_depth(&self) -> usize;
}

pub struct DepthFirstIter<T, P> {
    stack: VecStack<VecDeque<Strong<T>>>,
    curr_depth: usize,
    delve: P,
}

impl<T, P> DepthFirstIter<T, P> {
    fn new<'a, I>(root_layer: I, delve: P) -> Self
    where
        T: 'a,
        I: DoubleEndedIterator<Item = &'a StrongMut<T>>,
    {
        let stack = VecStack::from([root_layer.map(|x| x.clone()).collect()]);
        Self { stack, curr_depth: 0, delve }
    }

    #[inline]
    pub fn enumerate_depth(self) -> EnumerateDepth<Self> {
        EnumerateDepth::new(self)
    }
}

impl<T: Recursive, P: Fn(&T::Node) -> bool> Iterator for DepthFirstIter<T, P> {
    type Item = Strong<T>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(layer) = self.stack.top_mut() {
            if let Some(item) = layer.pop_front() {
                self.curr_depth = self.stack.len() - 1; // safety: could not get here if stack.len was 0
                if let Some(node) = item.read().get_if_node() {
                    if (self.delve)(node) {
                        self.stack.push(T::children(node).0.iter().map(|x| x.clone()).collect());
                    }
                }
                return Some(item);
            } else {
                _ = self.stack.pop();
            }
        }
        None
    }
}

impl<T: Recursive, P: Fn(&T::Node) -> bool> DoubleEndedIterator for DepthFirstIter<T, P> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(layer) = self.stack.top_mut() {
            if let Some(item) = layer.pop_back() {
                self.curr_depth = self.stack.len() - 1; // safety: could not get here if stack.len was 0
                if let Some(node) = item.read().get_if_node() {
                    if (self.delve)(node) {
                        self.stack.push(T::children(node).0.iter().map(|x| x.clone()).collect());
                    }
                }
                return Some(item);
            } else {
                _ = self.stack.pop();
            }
        }
        None
    }
}

impl<T: Recursive, P: Fn(&T::Node) -> bool> RecursiveIterator for DepthFirstIter<T, P> {
    type Inner = T;
    #[inline]
    fn last_depth(&self) -> usize {
        self.curr_depth
    }
}

pub struct DepthFirstIterMut<T, P> {
    stack: VecStack<VecDeque<StrongMut<T>>>,
    curr_depth: usize,
    delve: P,
}

impl<T, P> DepthFirstIterMut<T, P> {
    fn new<'a, I>(root_layer: I, delve: P) -> Self
    where
        T: 'a,
        I: DoubleEndedIterator<Item = &'a StrongMut<T>>,
    {
        let stack = VecStack::from([root_layer.map(|x| x.clone_mut()).collect()]);
        Self { stack, curr_depth: 0, delve }
    }

    #[inline]
    pub fn enumerate_depth(self) -> EnumerateDepth<Self> {
        EnumerateDepth::new(self)
    }
}

impl<T: Recursive, P: Fn(&T::Node) -> bool> Iterator for DepthFirstIterMut<T, P> {
    type Item = StrongMut<T>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(layer) = self.stack.top_mut() {
            if let Some(item) = layer.pop_front() {
                self.curr_depth = self.stack.len() - 1; // safety: could not get here if stack.len was 0
                if let Some(node) = item.read().get_if_node() {
                    if (self.delve)(node) {
                        self.stack.push(T::children(node).0.iter().map(|x| x.clone_mut()).collect());
                    }
                }
                return Some(item);
            } else {
                _ = self.stack.pop();
            }
        }
        None
    }
}

impl<T: Recursive, P: Fn(&T::Node) -> bool> DoubleEndedIterator for DepthFirstIterMut<T, P> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(layer) = self.stack.top_mut() {
            if let Some(item) = layer.pop_back() {
                self.curr_depth = self.stack.len() - 1; // safety: could not get here if stack.len was 0
                if let Some(node) = item.read().get_if_node() {
                    if (self.delve)(node) {
                        self.stack.push(T::children(node).0.iter().map(|x| x.clone_mut()).collect());
                    }
                }
                return Some(item);
            } else {
                _ = self.stack.pop();
            }
        }
        None
    }
}

impl<T: Recursive, P: Fn(&T::Node) -> bool> RecursiveIterator for DepthFirstIterMut<T, P> {
    type Inner = T;
    #[inline]
    fn last_depth(&self) -> usize {
        self.curr_depth
    }
}

pub struct EnumerateDepth<I> {
    iter: I,
}

impl<I> EnumerateDepth<I> {
    fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I: RecursiveIterator> Iterator for EnumerateDepth<I> {
    type Item = (usize, I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| (self.iter.last_depth(), x))
    }
}

impl<I: RecursiveIterator + DoubleEndedIterator> DoubleEndedIterator for EnumerateDepth<I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|x| (self.iter.last_depth(), x))
    }
}
