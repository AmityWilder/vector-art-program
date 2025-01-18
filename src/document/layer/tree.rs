use std::{collections::VecDeque, ops::{Deref, DerefMut}};
use raylib::prelude::*;
use crate::layer::{group::Group, Layer, LayerType, GAP};
use super::StrongLayer;

/// Front is background (bottom in layer panel) \
/// Back is foreground (top in layer panel)
///
/// Use `tree_iter()` to iterate recursively, `iter()` will only iterate over the current depth.
pub struct LayerTree(Vec<StrongLayer>);

impl Deref for LayerTree {
    type Target = Vec<StrongLayer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LayerTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Clone, Copy)]
pub enum LayerIterDir {
    /// Start at the foreground and traverse to the background
    ForeToBack,

    /// Start at the background and traverse to the foreground
    #[default]
    BackToFore,
}

#[allow(non_upper_case_globals)]
impl LayerIterDir {
    /// Start at the topmost layer in the layer panel and traverse to the bottom
    pub const TopToBot: Self = Self::ForeToBack;

    /// Start at the bottommost layer in the layer panel and traverse to the top
    pub const BotToTop: Self = Self::BackToFore;
}

pub struct LayerTreeIter<P: Fn(&Group) -> bool> {
    queue: VecDeque<(StrongLayer, usize)>,
    dir: LayerIterDir,
    should_recurs: P,
}

impl<P: Fn(&Group) -> bool> Iterator for LayerTreeIter<P> {
    type Item = (StrongLayer, usize);
    fn next(&mut self) -> Option<Self::Item> {
        self.queue
            .pop_front()
            .map(|(layer, depth)| {
                if let Layer::Group(group) = &*layer.read().expect("error handling not yet implemented") {
                    if (self.should_recurs)(group) {
                        self.queue.reserve(group.items.0.len());
                        let new_depth = depth + 1;
                        let items = group.items.0.iter().map(|item| (item.clone(), new_depth));
                        match self.dir {
                            LayerIterDir::ForeToBack => for item in items       { self.queue.push_front(item); },
                            LayerIterDir::BackToFore => for item in items.rev() { self.queue.push_front(item); }
                        }
                    }
                }
                (layer, depth)
            })
    }
}

impl LayerTree {
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
    pub fn tree_iter<P: Fn(&Group) -> bool>(&self, dir: LayerIterDir, should_recurs: P) -> LayerTreeIter<P> {
        let items = self.0.iter().map(|item| (item.clone(), 0));
        let queue = match dir {
            LayerIterDir::ForeToBack => VecDeque::from_iter(items.rev()),
            LayerIterDir::BackToFore => VecDeque::from_iter(items),
        };
        LayerTreeIter {
            queue,
            dir,
            should_recurs,
        }
    }

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
        for (layer, depth) in self.tree_iter(LayerIterDir::TopToBot, |group| group.is_expanded) {
            let mut layer = layer.write().expect("error handling not yet implemented");
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
