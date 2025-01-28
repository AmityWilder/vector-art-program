use std::collections::VecDeque;
use crate::rc::*;
use super::{EnumerateDepth, Recursive, RecursiveIterator, Tree};

impl<T: Recursive> Tree<T> {
    #[inline]
    pub fn bfs_iter<P: Fn(&T::Node) -> bool>(&self, delve: P) -> BreadthFirstIter<T, P> {
        BreadthFirstIter::new(self.0.iter(), delve)
    }
}

pub struct BreadthFirstIter<T, P> {
    queue: VecDeque<Strong<T>>,
    curr_depth_count: usize,
    curr_depth: usize,
    delve: P,
}

impl<T, P> BreadthFirstIter<T, P> {
    fn new<'a, I>(root_layer: I, delve: P) -> Self
    where
        T: 'a,
        I: DoubleEndedIterator<Item = &'a StrongMut<T>>,
    {
        let queue: VecDeque<_> = root_layer.map(|x| x.clone()).collect();
        let curr_depth_count = queue.len();
        Self { queue, curr_depth_count, curr_depth: 0, delve }
    }

    #[inline]
    pub fn enumerate_depth(self) -> EnumerateDepth<Self> {
        EnumerateDepth::new(self)
    }
}

impl<T: Recursive, P: Fn(&T::Node) -> bool> Iterator for BreadthFirstIter<T, P> {
    type Item = Strong<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.queue.pop_front() {
            if self.curr_depth_count == 0 {
                self.curr_depth += 1;
                self.curr_depth_count = self.queue.len();
            } else {
                self.curr_depth_count -= 1;
            }
            if let Some(node) = item.read().get_if_node() {
                if (self.delve)(node) {
                    self.queue.extend(T::children(node).0.iter().map(|x| x.clone()));
                }
            }
            return Some(item);
        }
        None
    }
}

impl<T: Recursive, P: Fn(&T::Node) -> bool> RecursiveIterator for BreadthFirstIter<T, P> {
    type Inner = T;
    #[inline]
    fn last_depth(&self) -> usize {
        self.curr_depth
    }
}

