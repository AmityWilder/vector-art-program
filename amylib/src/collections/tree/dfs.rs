use std::collections::VecDeque;
use crate::{collections::VecStack, rc::*};
use super::{EnumerateDepth, Recursive, RecursiveIterator, Tree};

impl<T: Recursive> Tree<T> {
    #[inline]
    pub fn dfs_iter<P: Fn(&T::Node) -> bool>(&self, delve: P) -> DepthFirstIter<T, P> {
        DepthFirstIter::new(self.0.iter(), delve)
    }

    #[inline]
    pub fn dfs_iter_mut<P: Fn(&T::Node) -> bool>(&mut self, delve: P) -> DepthFirstIterMut<T, P> {
        DepthFirstIterMut::new(self.0.iter(), delve)
    }
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
