use std::{collections::VecDeque, ops::{Deref, DerefMut}};
use raylib::prelude::*;
use crate::layer::{group::Group, Layer, LayerType, GAP};
use super::StrongLayer;

/// Front is background (bottom in layer panel) \
/// Back is foreground (top in layer panel)
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
pub enum LayerTreeDir {
    /// Start at the foreground and traverse to the background
    ForeToBack,

    /// Start at the background and traverse to the foreground
    #[default]
    BackToFore,
}

#[allow(non_upper_case_globals)]
impl LayerTreeDir {
    /// Start at the top in the layer panel and traverse to the bottom
    pub const TopToBot: Self = Self::ForeToBack;

    /// Start at the bottom in the layer panel and traverse to the top
    pub const BotToTop: Self = Self::BackToFore;
}

pub struct LayerTreeIter<P: Fn(&Group) -> bool> {
    queue: VecDeque<(StrongLayer, usize)>,
    dir: LayerTreeDir,
    should_recurs: P,
}

impl LayerTree {
    pub fn tree_iter<P: Fn(&Group) -> bool>(&self, dir: LayerTreeDir, should_recurs: P) -> LayerTreeIter<P> {
        let mut queue = VecDeque::with_capacity(self.0.len());
        match dir {
            LayerTreeDir::ForeToBack => for item in self.0.iter().rev() { queue.push_back((item.clone(), 0)) },
            LayerTreeDir::BackToFore => for item in self.0.iter()       { queue.push_back((item.clone(), 0)) },
        }
        LayerTreeIter {
            queue,
            dir,
            should_recurs,
        }
    }
}

impl<P: Fn(&Group) -> bool> Iterator for LayerTreeIter<P> {
    type Item = (StrongLayer, usize);
    fn next(&mut self) -> Option<Self::Item> {
        self.queue
            .pop_front()
            .map(|(layer, depth)| {
                if let Layer::Group(group) = &*layer.borrow() {
                    if (self.should_recurs)(group) {
                        match self.dir {
                            LayerTreeDir::ForeToBack => for item in group.items.iter()       { self.queue.push_front((item.clone(), depth + 1)); },
                            LayerTreeDir::BackToFore => for item in group.items.iter().rev() { self.queue.push_front((item.clone(), depth + 1)); }
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
        for (layer, depth) in self.tree_iter(LayerTreeDir::TopToBot, |group| group.is_expanded) {
            let mut layer = layer.borrow_mut();
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
