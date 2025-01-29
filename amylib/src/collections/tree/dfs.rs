use std::ptr::{null, null_mut};
use crate::collections::VecStack;
use super::{EnumerateDepth, Recursive, RecursiveIterator, Tree};

impl<T: Recursive> Tree<T> {
    #[inline]
    pub fn dfs_iter<P: Fn(&T::Node) -> bool>(&self, delve: P) -> DepthFirstIter<T, P> {
        DepthFirstIter::new(self.0.iter(), delve)
    }

    #[inline]
    pub fn dfs_iter_mut<P: Fn(&T::Node) -> bool>(&mut self, delve: P) -> DepthFirstIterMut<T, P> {
        DepthFirstIterMut::new(self.0.iter_mut(), delve)
    }
}

pub struct DepthFirstIter<'a, T: 'a, P> {
    /// ## Safety
    /// We don't actually move our item when returning,
    /// we just give the caller a reference to it.
    ///
    /// By holding onto a pointer, we are simply keeping
    /// a reference to the memory that *won't get mutated*
    /// by this iterator until the reference we returned is
    /// dropped by the caller ...I think.
    ///
    /// I'm not sure how to specify "the mutable reference
    /// must be within a scope that ends **before**
    /// [`Iterator::next`] is called again"
    ///
    /// It definitely shouldn't get *dropped* before the
    /// iterator uses it again at least, so there's that.
    on_deck: *const T,
    stack: VecStack<std::slice::Iter<'a, T>>,
    delve: P,
}

impl<'a, T: 'a, P> DepthFirstIter<'a, T, P> {
    fn new(root_layer: std::slice::Iter<'a, T>, delve: P) -> Self {
        let stack = VecStack::from([root_layer]);
        Self { on_deck: null(), stack, delve }
    }

    #[inline]
    pub fn enumerate_depth(self) -> EnumerateDepth<Self> {
        EnumerateDepth::new(self)
    }
}

impl<'a, T: 'a + Recursive, P: Fn(&T::Node) -> bool> Iterator for DepthFirstIter<'a, T, P> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.on_deck.is_null() {
            let item: &'a T = unsafe { &*self.on_deck };
            self.on_deck = null_mut();
            if let Some(node) = item.get_if_node() {
                if (self.delve)(node) {
                    self.stack.push(T::children(node).0.iter());
                }
            }
        }
        while let Some(layer) = self.stack.top_mut() {
            if let Some(item) = layer.next() {
                self.on_deck = item as *const T;
                return Some(item);
            } else {
                _ = self.stack.pop();
            }
        }
        None
    }
}

impl<'a, T: 'a + Recursive, P: Fn(&T::Node) -> bool> DoubleEndedIterator for DepthFirstIter<'a, T, P> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if !self.on_deck.is_null() {
            let item: &'a T = unsafe { &*self.on_deck };
            self.on_deck = null();
            if let Some(node) = item.get_if_node() {
                if (self.delve)(node) {
                    self.stack.push(T::children(node).0.iter());
                }
            }
        }
        while let Some(layer) = self.stack.top_mut() {
            if let Some(item) = layer.next_back() {
                self.on_deck = item as *const T;
                return Some(item);
            } else {
                _ = self.stack.pop();
            }
        }
        None
    }
}

impl<'a, T: 'a + Recursive, P: Fn(&T::Node) -> bool> RecursiveIterator for DepthFirstIter<'a, T, P> {
    #[inline]
    fn depth(&self) -> usize {
        self.stack.len().saturating_sub(1)
    }
}

pub struct DepthFirstIterMut<'a, T: 'a, P> {
    /// ## Safety
    /// We don't actually move our item when returning,
    /// we just give the caller a reference to it.
    ///
    /// By holding onto a pointer, we are simply keeping
    /// a reference to the memory that *won't get mutated*
    /// by this iterator until the reference we returned is
    /// dropped by the caller ...I think.
    ///
    /// I'm not sure how to specify "the mutable reference
    /// must be within a scope that ends **before**
    /// [`Iterator::next`] is called again"
    ///
    /// It definitely shouldn't get *dropped* before the
    /// iterator uses it again at least, so there's that.
    on_deck: *mut T,
    stack: VecStack<std::slice::IterMut<'a, T>>,
    delve: P,
}

impl<'a, T: 'a, P> DepthFirstIterMut<'a, T, P> {
    fn new(root_layer: std::slice::IterMut<'a, T>, delve: P) -> Self {
        let stack = VecStack::from([root_layer]);
        Self { on_deck: null_mut(), stack, delve }
    }

    #[inline]
    pub fn enumerate_depth(self) -> EnumerateDepth<Self> {
        EnumerateDepth::new(self)
    }
}

impl<'a, T: 'a + Recursive, P: Fn(&T::Node) -> bool> Iterator for DepthFirstIterMut<'a, T, P> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.on_deck.is_null() {
            let item: &'a mut T = unsafe { &mut *self.on_deck };
            self.on_deck = null_mut();
            if let Some(node) = item.get_if_node_mut() {
                if (self.delve)(node) {
                    self.stack.push(T::children_mut(node).0.iter_mut());
                }
            }
        }
        while let Some(layer) = self.stack.top_mut() {
            if let Some(item) = layer.next() {
                self.on_deck = item as *mut T;
                return Some(item);
            } else {
                _ = self.stack.pop();
            }
        }
        None
    }
}

impl<'a, T: 'a + Recursive, P: Fn(&T::Node) -> bool> DoubleEndedIterator for DepthFirstIterMut<'a, T, P> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if !self.on_deck.is_null() {
            let item: &'a mut T = unsafe { &mut *self.on_deck };
            if let Some(node) = item.get_if_node_mut() {
                if (self.delve)(node) {
                    self.stack.push(T::children_mut(node).0.iter_mut());
                }
            }
        }
        while let Some(layer) = self.stack.top_mut() {
            if let Some(item) = layer.next_back() {
                self.on_deck = item as *mut T;
                return Some(item);
            } else {
                _ = self.stack.pop();
            }
        }
        None
    }
}

impl<'a, T: 'a + Recursive, P: Fn(&T::Node) -> bool> RecursiveIterator for DepthFirstIterMut<'a, T, P> {
    #[inline]
    fn depth(&self) -> usize {
        self.stack.len().saturating_sub(1)
    }
}
