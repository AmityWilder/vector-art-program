use std::collections::VecDeque;
use super::{EnumerateDepth, Recursive, RecursiveIterator};

pub struct BreadthFirstIter<'a, T: 'a, P> {
    queue: VecDeque<&'a T>,
    curr_depth_count: usize,
    curr_depth: usize,
    delve: P,
}

impl<'a, T: 'a, P> BreadthFirstIter<'a, T, P> {
    pub(super) fn new(root_layer: std::slice::Iter<'a, T>, delve: P) -> Self {
        let queue = VecDeque::from_iter(root_layer);
        let curr_depth_count = queue.len();
        Self { queue, curr_depth_count, curr_depth: 0, delve }
    }

    #[inline]
    pub fn enumerate_depth(self) -> EnumerateDepth<Self> {
        EnumerateDepth::new(self)
    }
}

impl<'a, T: 'a + Recursive, P: Fn(&T::Node) -> bool> Iterator for BreadthFirstIter<'a, T, P> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.queue.pop_front() {
            if self.curr_depth_count == 0 {
                self.curr_depth += 1;
                self.curr_depth_count = self.queue.len();
            } else {
                self.curr_depth_count -= 1;
            }
            if let Some(node) = item.if_node() {
                if (self.delve)(node) {
                    self.queue.extend(T::children(node).0.iter());
                }
            }
            return Some(item);
        }
        None
    }
}

impl<'a, T: 'a + Recursive, P: Fn(&T::Node) -> bool> RecursiveIterator for BreadthFirstIter<'a, T, P> {
    #[inline]
    fn depth(&self) -> usize {
        self.curr_depth
    }
}

