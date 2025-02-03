use amymath::prelude::*;
use crate::document::Document;
use amylib::iter::directed::DirectibleDoubleEndedIterator;
use super::{Layer, TopToBot};

pub const INSET: i32 = 2;
pub const GAP: i32 = 2;
pub const INDENT: i32 = 6;
pub const THUMBNAIL_SIZE: i32 = 32;
pub const THUMBNAIL_INSET: i32 = 6;
pub const LAYER_HEIGHT: i32 = 2 * THUMBNAIL_INSET + THUMBNAIL_SIZE;
pub const LAYER_COLOR_WIDTH: i32 = 4;
pub const TEXT_FONT_SIZE: i32 = 10;
pub const EXPAND_COLLAPSE_SIZE: i32 = 10;

#[derive(Default)]
pub struct LayerUIRecs {
    /// The rectangle representing the entire layer slot
    pub slot: IRect2,
    /// The rectangle representing the layer color slice
    pub color: IRect2,
    /// The rectangle representing the layer thumbnail
    pub thumbnail: IRect2,
    /// The rectangle containing the layer name
    pub name: IRect2,
    /// The rectangle representing the expand/collapse button on groups
    pub expand_button: Option<IRect2>,
}

impl LayerUIRecs {
    pub fn generate(mut rec: IRect2, is_group: bool) -> Self {
        let width = rec.width();
        let slot_rec = rec;
        let color_rec = {
            rec.xmax = rec.xmin + LAYER_COLOR_WIDTH;
            rec
        };
        let thumbnail_rec = {
            rec.xmin += LAYER_COLOR_WIDTH + THUMBNAIL_INSET;
            rec.ymin += THUMBNAIL_INSET;
            (rec.xmax, rec.ymax) = (rec.xmin + THUMBNAIL_SIZE, rec.ymin + THUMBNAIL_SIZE);
            rec
        };
        let name_rec = {
            rec.xmin += THUMBNAIL_SIZE + THUMBNAIL_INSET;
            rec.ymax = rec.ymin + TEXT_FONT_SIZE;
            rec.xmax = rec.xmin + width - LAYER_COLOR_WIDTH + THUMBNAIL_INSET - THUMBNAIL_SIZE;
            rec
        };
        let expand_button_rec = is_group.then(|| {
            rec.ymin += TEXT_FONT_SIZE + INSET;
            rec.xmax = rec.xmin + EXPAND_COLLAPSE_SIZE;
            rec.ymax = rec.ymin + EXPAND_COLLAPSE_SIZE;
            rec
        });
        LayerUIRecs {
            slot: slot_rec,
            color: color_rec,
            thumbnail: thumbnail_rec,
            name: name_rec,
            expand_button: expand_button_rec,
        }
    }
}

pub struct LayerUiIter<I> {
    tree_iter: I,
    container: IRect2,
    slot: IRect2,
}

trait IsGroup {
    fn is_group(&self) -> bool;
}
impl IsGroup for &Layer {
    #[inline]
    fn is_group(&self) -> bool {
        matches!(self, Layer::Group(_))
    }
}
impl IsGroup for &mut Layer {
    #[inline]
    fn is_group(&self) -> bool {
        matches!(self, Layer::Group(_))
    }
}

impl<L: IsGroup, I: Iterator<Item = (usize, L)>> Iterator for LayerUiIter<I> {
    type Item = (L, LayerUIRecs);

    fn next(&mut self) -> Option<Self::Item> {
        let container = &self.container;
        for (depth, layer) in self.tree_iter.by_ref() {
            let ymin = self.slot.ymin;
            if ymin >= container.ymax {
                return None; // no more are going to be visible
            }
            let bottom = self.slot.ymin + LAYER_HEIGHT + GAP;
            if bottom < container.ymin {
                self.slot.ymin = bottom;
                continue; // sprint to first visible
            }
            let indentation =
                #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation, reason = "guarded by `min(256)`")]
                (depth.min(256) as i32) * INDENT;
            self.slot.xmin = container.xmin + indentation;
            self.slot.xmax = container.xmax - indentation;
            let recs = self.slot.is_overlapping(container)
                .then(|| LayerUIRecs::generate(self.slot, layer.is_group()));
            self.slot.ymin = bottom;
            if let Some(recs) = recs {
                return Some((layer, recs));
            }
        }
        None
    }
}

impl Document {
    /// Iterate over each expanded layer panel item immutably, overlapping `container`, with the first item's y-value being `top`
    pub fn ui_iter(&self, container: &IRect2, top: i32) -> impl Iterator<Item = (&Layer, LayerUIRecs)> {
        LayerUiIter {
            tree_iter: self.layers
                .dfs_iter(|group| group.is_expanded)
                .enumerate_depth()
                .cdir::<TopToBot>(),
            slot: IRect2 {
                xmin: container.xmin,
                ymin: top + INSET,
                xmax: container.xmax,
                ymax: top + INSET + LAYER_HEIGHT,
            },
            container: *container,
        }
    }

    /// Iterate over each expanded layer panel item mutably, overlapping `container`, with the first item's y-value being `top`
    pub fn ui_iter_mut(&mut self, container: &IRect2, top: i32) -> impl Iterator<Item = (&mut Layer, LayerUIRecs)> {
        LayerUiIter {
            tree_iter: self.layers
                .dfs_iter_mut(|group| group.is_expanded)
                .enumerate_depth()
                .cdir::<TopToBot>(),
            slot: IRect2 {
                xmin: container.xmin,
                ymin: top + INSET,
                xmax: container.xmax,
                ymax: top + INSET + LAYER_HEIGHT,
            },
            container: *container,
        }
    }
}
