use std::{collections::VecDeque, ops::{Deref, DerefMut}};
use crate::rc::{Strong, StrongMut};
use super::stack::VecStack;

pub trait Recursive: Sized {
    type Node;
    fn get_if_node(&self) -> Option<&Self::Node>;
    fn get_if_node_mut(&mut self) -> Option<&mut Self::Node>;
    fn children(node: &Self::Node) -> &Tree<Self>;
    fn children_mut(node: &mut Self::Node) -> &mut Tree<Self>;
}

/// Front is background (bottom in layer panel) \
/// Back is foreground (top in layer panel)
///
/// Use `tree_iter()` to iterate recursively, `iter()` will only iterate over the current depth.
pub struct Tree<T: Recursive>(Vec<StrongMut<T>>);

impl<T: Recursive> Deref for Tree<T> {
    type Target = Vec<StrongMut<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Recursive> DerefMut for Tree<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum TreeIterDir {
    /// The order elements are stored in the tree vec
    #[default]
    Forward,
    Reverse,
}

pub struct DepthFirstIter<T: Recursive, P: Fn(&T::Node) -> bool> {
    stack: VecStack<VecDeque<Strong<T>>>,
    dir: TreeIterDir,
    delve: P,
}

impl<T: Recursive, P: Fn(&T::Node) -> bool> Iterator for DepthFirstIter<T, P> {
    type Item = (usize, Strong<T>);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(layer) = self.stack.top_mut() {
            if let Some(item) = layer.pop_front() {
                let depth = self.stack.len() - 1; // safety: could not get here if stack.len was 0
                if let Some(node) = item.read().get_if_node() {
                    if (self.delve)(node) {
                        let cloned = T::children(node).0.iter().map(|x| x.clone());
                        self.stack.push(match self.dir {
                            TreeIterDir::Forward => cloned      .collect(),
                            TreeIterDir::Reverse => cloned.rev().collect(),
                        });
                    }
                }
                return Some((depth, item));
            } else {
                _ = self.stack.pop();
            }
        }
        None
    }
}

pub struct TreeIterMut<T: Recursive, P: Fn(&T::Node) -> bool> {
    stack: VecStack<VecDeque<StrongMut<T>>>,
    dir: TreeIterDir,
    delve: P,
}

impl<T: Recursive, P: Fn(&T::Node) -> bool> Iterator for TreeIterMut<T, P> {
    type Item = (usize, StrongMut<T>);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(layer) = self.stack.top_mut() {
            if let Some(item) = layer.pop_front() {
                let depth = self.stack.len() - 1; // safety: could not get here if stack.len was 0
                if let Some(node) = item.read().get_if_node() {
                    if (self.delve)(node) {
                        let cloned = T::children(node).0.iter().map(|x| x.clone_mut());
                        self.stack.push(match self.dir {
                            TreeIterDir::Forward => cloned      .collect(),
                            TreeIterDir::Reverse => cloned.rev().collect(),
                        });
                    }
                }
                return Some((depth, item));
            } else {
                _ = self.stack.pop();
            }
        }
        None
    }
}

impl<T: Recursive> Tree<T> {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Use a DFS algorithm to traverse the layer tree.
    ///
    /// ---
    ///
    /// `should_recurse` determines whether a branch's children should be visited or skipped.
    ///
    /// ---
    ///
    /// `dir` determines what order the layers are visited in: front to back or back to front.
    ///
    /// Parent is visited before child regardless of direction.
    ///
    /// ---
    ///
    /// Iterator returns elements in a tuple of
    /// ```no_run
    /// (layer: StrongLayer, depth: usize)
    /// ```
    pub fn tree_iter<P: Fn(&T::Node) -> bool>(&self, dir: TreeIterDir, should_recurs: P) -> DepthFirstIter<T, P> {
        let iter = self.0.iter().map(|x| x.clone());
        let stack = match dir {
            TreeIterDir::Reverse => VecStack::from([iter.rev().collect()]),
            TreeIterDir::Forward => VecStack::from([iter      .collect()]),
        };
        DepthFirstIter {
            stack,
            dir,
            delve: should_recurs,
        }
    }

    /// See [`Self::tree_iter`]
    pub fn tree_iter_mut<P: Fn(&T::Node) -> bool>(&mut self, dir: TreeIterDir, should_recurs: P) -> TreeIterMut<T, P> {
        let iter = self.0.iter().map(|x| x.clone_mut());
        let stack = match dir {
            TreeIterDir::Reverse => VecStack::from([iter.rev().collect()]),
            TreeIterDir::Forward => VecStack::from([iter      .collect()]),
        };
        TreeIterMut {
            stack,
            dir,
            delve: should_recurs,
        }
    }
}
