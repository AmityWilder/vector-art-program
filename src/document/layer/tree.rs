use std::ops::{Deref, DerefMut};

use raylib::prelude::*;
use crate::layer::{group::Group, Layer, GAP};
use super::{LayerType, StrongLayer};

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

impl LayerTree {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Each item that is visible in the layer panel
    pub fn foreach_layer_tree_item<'a, T>(&self, mut f: impl 'a + FnMut(&Layer, usize) -> Option<T>) -> Option<T> {
        fn recursive<T>(layers: &LayerTree, depth: usize, f: &mut impl FnMut(&Layer, usize) -> Option<T>) -> Option<T> {
            for layer in layers.0.iter().rev() {
                let layer = layer.borrow();
                f(&layer, depth);
                if let Layer::Group(Group { group, is_expanded: true, .. }) = &*layer {
                    let result = recursive(group, depth + 1, f);
                    if result.is_some() { return result; }
                }
            }
            None
        }

        recursive(self, 0, &mut f)
    }

    /// Each item that is visible in the layer panel
    pub fn foreach_layer_tree_item_mut<'a, T>(&mut self, mut f: impl 'a + FnMut(&mut Layer, usize) -> Option<T>) -> Option<T> {
        fn recursive<T>(layers: &mut LayerTree, depth: usize, f: &mut impl FnMut(&mut Layer, usize) -> Option<T>) -> Option<T> {
            for layer in layers.0.iter().rev() {
                let mut layer = layer.borrow_mut();
                f(&mut layer, depth);
                if let Layer::Group(Group { group, is_expanded: true, .. }) = &mut *layer {
                    let result = recursive(group, depth + 1, f);
                    if result.is_some() { return result; }
                }
            }
            None
        }

        recursive(self, 0, &mut f)
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
        self.foreach_layer_tree_item_mut(|layer, depth| -> Option<()> {
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
            match &mut *layer {
                Layer::Group(Group { expand_button_rec, .. }) => {
                    *expand_button_rec = {
                        rec.y += TEXT_FONT_SIZE + 2.0;
                        rec.width  = EXPAND_COLLAPSE_SIZE;
                        rec.height = EXPAND_COLLAPSE_SIZE;
                        rec
                    };
                }
                _ => (),
            }
            None
        });
    }
}
