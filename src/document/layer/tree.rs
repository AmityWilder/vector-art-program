use std::{collections::VecDeque, ops::{Deref, DerefMut}};
use raylib::prelude::*;
use crate::layer::{group::Group, Layer, LayerType, GAP};
use super::rc::*;

pub trait Recursive: Sized {
    type Node;
    fn get_if_node(&self) -> Option<&Self::Node>;
    fn get_if_node_mut(&mut self) -> Option<&mut Self::Node>;
    fn children(node: &Self::Node) -> &Tree<Self>;
    fn children_mut(node: &mut Self::Node) -> &mut Tree<Self>;
}

impl Recursive for Layer {
    type Node = Group;
    fn get_if_node(&self) -> Option<&Self::Node> {
        if let Self::Group(group) = self { Some(group) } else { None }
    }
    fn get_if_node_mut(&mut self) -> Option<&mut Self::Node> {
        if let Self::Group(group) = self { Some(group) } else { None }
    }
    fn children(node: &Self::Node) -> &Tree<Self> {
        &node.items
    }
    fn children_mut(node: &mut Self::Node) -> &mut Tree<Self> {
        &mut node.items
    }
}

/// Front is background (bottom in layer panel) \
/// Back is foreground (top in layer panel)
///
/// Use `tree_iter()` to iterate recursively, `iter()` will only iterate over the current depth.
pub struct Tree<T: Recursive>(Vec<StrongMut<T>>);
pub type LayerTree = Tree<Layer>;

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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TreeIterDir {
    Forward,
    Reverse,
}

impl Default for TreeIterDir {
    /// The order layers are stored in the tree vec
    fn default() -> Self {
        Self::Forward
    }
}

#[allow(non_upper_case_globals)]
impl TreeIterDir {
    /// Start at the background and traverse to the foreground
    ///
    /// Visit elements in the order they should be drawn so they occlude each other correctly
    pub const BackToFore: Self = Self::Forward;

    /// Start at the foreground and traverse to the background
    ///
    /// Visit elements in the order that mouse collisions should find them
    pub const ForeToBack: Self = Self::Reverse;

    /// Start at the bottommost layer in the layer panel and traverse to the top
    ///
    /// Reverse of `TopToBot` for sake of consistency - I haven't found a use for this yet
    pub const BotToTop: Self = Self::Forward;

    /// Start at the topmost layer in the layer panel and traverse to the bottom
    ///
    /// Visit elements in the order their height influences following layers
    pub const TopToBot: Self = Self::Reverse;
}

pub struct TreeIter<T: Recursive, P: Fn(&T::Node) -> bool> {
    queue: VecDeque<(Strong<T>, usize)>,
    dir: TreeIterDir,
    should_recurs: P,
}

impl<T: Recursive, P: Fn(&T::Node) -> bool> Iterator for TreeIter<T, P> {
    type Item = (Strong<T>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        self.queue
            .pop_front()
            .map(|(item, depth)| {
                if let Some(node) = item.read().get_if_node() {
                    if (self.should_recurs)(node) {
                        let Tree(subtree) = T::children(node);
                        self.queue.reserve(subtree.len());
                        let new_depth = depth + 1;
                        let items = subtree.iter().map(|item| (item.clone(), new_depth));
                        match self.dir {
                            TreeIterDir::ForeToBack => for item in items       { self.queue.push_front(item); },
                            TreeIterDir::BackToFore => for item in items.rev() { self.queue.push_front(item); }
                        }
                    }
                }
                (item, depth)
            })
    }
}

pub struct TreeIterMut<T: Recursive, P: Fn(&T::Node) -> bool> {
    queue: VecDeque<(StrongMut<T>, usize)>,
    dir: TreeIterDir,
    should_recurs: P,
}

impl<T: Recursive, P: Fn(&T::Node) -> bool> Iterator for TreeIterMut<T, P> {
    type Item = (StrongMut<T>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        self.queue
            .pop_front()
            .map(|(mut item, depth)| {
                if let Some(node) = item.write().get_if_node_mut() {
                    if (self.should_recurs)(node) {
                        let Tree(subtree) = T::children_mut(node);
                        self.queue.reserve(subtree.len());
                        let new_depth = depth + 1;
                        let items = subtree.iter().map(|item| (item.clone_mut(), new_depth));
                        match self.dir {
                            TreeIterDir::ForeToBack => for item in items       { self.queue.push_front(item); },
                            TreeIterDir::BackToFore => for item in items.rev() { self.queue.push_front(item); }
                        }
                    }
                }
                (item, depth)
            })
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
    /// e.g.
    ///
    /// [`LayerIterDir::BackToFore`]:
    /// - 0
    ///   - 0.0
    ///   - 0.1
    ///   - 0.2
    /// - 1
    /// - 2
    ///   - 2.1
    /// - 3
    /// - 4
    /// - 5
    ///   - 5.0
    ///     - 5.0.0
    ///   - 5.1
    ///   - 5.2
    ///     - 5.2.0
    ///     - 5.2.1
    ///
    /// [`LayerIterDir::ForeToBack`]:
    /// - 5
    ///   - 5.2
    ///     - 5.2.1
    ///     - 5.2.0
    ///   - 5.1
    ///   - 5.0
    ///     - 5.0.0
    /// - 4
    /// - 3
    /// - 2
    ///   - 2.1
    /// - 1
    /// - 0
    ///   - 0.2
    ///   - 0.1
    ///   - 0.0
    ///
    /// Notice that in both cases, 5 is visited before 5.0 and 5.0 is visited before 5.0.0. \
    /// But in one, 5.0 comes before 5.1; while in the other, 5.1 comes before 5.0.
    ///
    /// ---
    ///
    /// Iterator returns elements in a tuple of
    /// ```no_run
    /// (layer: StrongLayer, depth: usize)
    /// ```
    pub fn tree_iter<P: Fn(&T::Node) -> bool>(&self, dir: TreeIterDir, should_recurs: P) -> TreeIter<T, P> {
        let items = self.0.iter().map(|item| (item.clone(), 0));
        let queue = match dir {
            TreeIterDir::ForeToBack => VecDeque::from_iter(items.rev()),
            TreeIterDir::BackToFore => VecDeque::from_iter(items),
        };
        TreeIter {
            queue,
            dir,
            should_recurs,
        }
    }

    /// See [`Self::tree_iter`]
    pub fn tree_iter_mut<P: Fn(&T::Node) -> bool>(&mut self, dir: TreeIterDir, should_recurs: P) -> TreeIterMut<T, P> {
        let items = self.0.iter().map(|item| (item.clone_mut(), 0));
        let queue = match dir {
            TreeIterDir::ForeToBack => VecDeque::from_iter(items.rev()),
            TreeIterDir::BackToFore => VecDeque::from_iter(items),
        };
        TreeIterMut {
            queue,
            dir,
            should_recurs,
        }
    }
}

impl Tree<Layer> {
    pub fn update_ui_recs(&mut self, container: &Rectangle, mut top: f32) {
        use super::{
            INDENT,
            THUMBNAIL_SIZE,
            THUMBNAIL_INSET,
            LAYER_HEIGHT,
            LAYER_COLOR_WIDTH,
            TEXT_FONT_SIZE,
            EXPAND_COLLAPSE_SIZE,
        };
        for (mut layer, depth) in self.tree_iter_mut(TreeIterDir::TopToBot, |group| group.is_expanded) {
            let mut layer = layer.write();
            let settings = layer.settings_mut();
            let indent_size = depth as f32 * INDENT;
            let left = container.x + indent_size;
            let width = container.width - indent_size;
            let mut rec = Rectangle::new(left, top, width, LAYER_HEIGHT);
            settings.slot_rec = rec;
            settings.color_rec = {
                rec.width = LAYER_COLOR_WIDTH;
                rec
            };
            settings.thumbnail_rec = {
                rec.x += LAYER_COLOR_WIDTH + THUMBNAIL_INSET;
                rec.y += THUMBNAIL_INSET;
                (rec.width, rec.height) = (THUMBNAIL_SIZE, THUMBNAIL_SIZE);
                rec
            };
            settings.name_rec = {
                rec.x += THUMBNAIL_SIZE + THUMBNAIL_INSET;
                rec.height = TEXT_FONT_SIZE;
                rec.width = width - LAYER_COLOR_WIDTH + THUMBNAIL_INSET - THUMBNAIL_SIZE;
                rec
            };
            top += LAYER_HEIGHT + GAP;
            if let Layer::Group(Group { expand_button_rec, .. }) = &mut *layer {
                *expand_button_rec = {
                    rec.y += TEXT_FONT_SIZE + 2.0;
                    rec.width  = EXPAND_COLLAPSE_SIZE;
                    rec.height = EXPAND_COLLAPSE_SIZE;
                    rec
                };
            }
        }
    }
}
