use raylib::prelude::*;
use crate::ui::panel::Rect2;
use amylib::{rc::*, collections::tree::*};
use super::{group::Group, Layer, LayerTree, TopToBot};

pub const INSET: f32 = 2.0;
pub const GAP: f32 = 2.0;
pub const INDENT: f32 = 6.0;
pub const THUMBNAIL_SIZE: f32 = 32.0;
pub const THUMBNAIL_INSET: f32 = 6.0;
pub const LAYER_HEIGHT: f32 = 2.0 * THUMBNAIL_INSET + THUMBNAIL_SIZE;
pub const LAYER_COLOR_WIDTH: f32 = 4.0;
pub const TEXT_FONT_SIZE: f32 = 10.0;
pub const EXPAND_COLLAPSE_SIZE: f32 = 10.0;

#[derive(Default)]
pub struct LayerUI {
    pub slot_rec: Rectangle,
    pub color_rec: Rectangle,
    pub thumbnail_rec: Rectangle,
    pub name_rec: Rectangle,
    pub expand_button_rec: Option<Rectangle>,
}

impl LayerUI {
    pub fn generate(mut rec: Rectangle, is_group: bool) -> Self {
        let width = rec.width;
        let slot_rec = rec;
        let color_rec = {
            rec.width = LAYER_COLOR_WIDTH;
            rec
        };
        let thumbnail_rec = {
            rec.x += LAYER_COLOR_WIDTH + THUMBNAIL_INSET;
            rec.y += THUMBNAIL_INSET;
            (rec.width, rec.height) = (THUMBNAIL_SIZE, THUMBNAIL_SIZE);
            rec
        };
        let name_rec = {
            rec.x += THUMBNAIL_SIZE + THUMBNAIL_INSET;
            rec.height = TEXT_FONT_SIZE;
            rec.width = width - LAYER_COLOR_WIDTH + THUMBNAIL_INSET - THUMBNAIL_SIZE;
            rec
        };
        let expand_button_rec = is_group.then(|| {
            rec.y += TEXT_FONT_SIZE + 2.0;
            rec.width  = EXPAND_COLLAPSE_SIZE;
            rec.height = EXPAND_COLLAPSE_SIZE;
            rec
        });
        LayerUI {
            slot_rec,
            color_rec,
            thumbnail_rec,
            name_rec,
            expand_button_rec,
        }
    }
}

pub struct LayerTreeUiIter<T: Owned<Layer>, I: Iterator<Item = (T, usize)>> {
    tree_iter: I,
    container: Rectangle,
    slot: Rectangle,
}

impl<T: Owned<Layer>, I: Iterator<Item = (T, usize)>> Iterator for LayerTreeUiIter<T, I> {
    type Item = (T, usize, LayerUI);
    fn next(&mut self) -> Option<Self::Item> {
        let container = &self.container;
        let container_bottom = container.y + container.height;
        while let Some((layer, depth)) = self.tree_iter.next() {
            let y = self.slot.y;
            if y >= container_bottom {
                return None; // no more are going to be visible
            }
            let bottom = self.slot.y + LAYER_HEIGHT + GAP;
            if bottom < container.y {
                self.slot.y = bottom;
                continue; // sprint to first visible
            }
            let indentation = depth as f32 * INDENT;
            self.slot.x = container.x + indentation;
            self.slot.width = container.width - indentation;
            if self.slot.check_collision_recs(container) {
                let is_group = matches!(&*layer.read(), Layer::Group(_));
                let recs = LayerUI::generate(self.slot, is_group);
                self.slot.y = bottom;
                return Some((layer, depth, recs));
            } else {
                self.slot.y = bottom;
            }
        }
        None
    }
}

pub trait LayerUiIter {
    fn ui_iter(&self, container: Rect2, top: f32) -> LayerTreeUiIter<Strong<Layer>, TreeIter<Layer, impl Fn(&Group) -> bool>>;
    fn ui_iter_mut(&mut self, container: Rect2, top: f32) -> LayerTreeUiIter<StrongMut<Layer>, TreeIterMut<Layer, impl Fn(&Group) -> bool>>;
}

impl LayerUiIter for LayerTree {
    /// Iterate over each expanded layer panel item immutably, overlapping `container`, with the first item's y-value being `top`
    fn ui_iter(&self, container: Rect2, top: f32) -> LayerTreeUiIter<Strong<Layer>, TreeIter<Layer, impl Fn(&Group) -> bool>> {
        let container = container.into();
        LayerTreeUiIter {
            tree_iter: self.tree_iter(TopToBot, |group| group.is_expanded),
            container,
            slot: Rectangle {
                x: container.x,
                y: top + INSET,
                width: container.width,
                height: LAYER_HEIGHT,
            },
        }
    }

    /// Iterate over each expanded layer panel item mutably, overlapping `container`, with the first item's y-value being `top`
    fn ui_iter_mut(&mut self, container: Rect2, top: f32) -> LayerTreeUiIter<StrongMut<Layer>, TreeIterMut<Layer, impl Fn(&Group) -> bool>> {
        let container = container.into();
        LayerTreeUiIter {
            tree_iter: self.tree_iter_mut(TopToBot, |group| group.is_expanded),
            container,
            slot: Rectangle {
                x: container.x,
                y: top + INSET,
                width: container.width,
                height: LAYER_HEIGHT,
            },
        }
    }
}
