use std::{cell::RefCell, rc::Rc};
use raylib::prelude::*;
use crate::{Bitmap, appearance::Blending, VectorPath};
pub enum LayerItem {
    Group(Rc<RefCell<Layer>>),
    Path(Rc<RefCell<VectorPath>>),
    Bitmap(Bitmap),
}

pub struct Layer {
    /// Name of the layer in the layers panel
    pub name: String,
    /// Color of paths
    pub color: Color,
    /// Skip in rendering
    pub is_hidden: bool,
    /// Disallow selection and editing
    pub is_locked: bool,
    /// Show items in layer tree
    pub is_expanded: bool,
    /// Items move with layer
    pub is_group: bool,
    pub blend: Blending,
    pub bounds: Rectangle,
    pub items: Vec<Rc<RefCell<LayerItem>>>,
}

pub enum LayerPanelTreeItemEx<'a> {
    Layer {
        layer: &'a mut Layer,
    },
    Path {
        layer: &'a Layer,
        path: &'a mut Rc<RefCell<VectorPath>>,
    },
    Bitmap {
        layer: &'a Layer,
        bitmap: &'a mut Bitmap,
    },
}

pub struct LayerPanelTreeItem<'a> {
    pub slot: Rectangle,
    pub color_rec: Rectangle,
    pub thumbnail_rec: Rectangle,
    pub name_rec: Rectangle,
    pub expand_collapse_rec: Rectangle,
    pub ex: LayerPanelTreeItemEx<'a>,
}

impl<'a> LayerPanelTreeItem<'a> {
    pub fn new(
        mut rec: Rectangle,
        ex: LayerPanelTreeItemEx<'a>,
    ) -> Self {
        let width = rec.width;
        Self {
            slot: rec,
            color_rec: {
                rec.width = Layer::LAYER_COLOR_WIDTH;
                rec
            },
            thumbnail_rec: {
                rec.x += Layer::LAYER_COLOR_WIDTH + Layer::THUMBNAIL_INSET;
                rec.y += Layer::THUMBNAIL_INSET;
                (rec.width, rec.height) = (Layer::THUMBNAIL_SIZE, Layer::THUMBNAIL_SIZE);
                rec
            },
            name_rec: {
                rec.x += Layer::THUMBNAIL_SIZE + Layer::THUMBNAIL_INSET;
                rec.height = Layer::TEXT_FONT_SIZE;
                rec.width = width - Layer::LAYER_COLOR_WIDTH + Layer::THUMBNAIL_INSET - Layer::THUMBNAIL_SIZE;
                rec
            },
            expand_collapse_rec: {
                rec.y += Layer::TEXT_FONT_SIZE + 2.0;
                rec.width  = Layer::EXPAND_COLLAPSE_SIZE;
                rec.height = Layer::EXPAND_COLLAPSE_SIZE;
                rec
            },
            ex,
        }
    }
}

impl Layer {
    pub fn new(name: String, color: Color) -> Self {
        Self {
            name,
            color,
            is_hidden: false,
            is_locked: false,
            is_expanded: false,
            is_group: false,
            blend: Blending::default(),
            bounds: Rectangle::default(),
            items: Vec::new(),
        }
    }

    pub fn draw_rendered(&self, d: &mut impl RaylibDraw) {
        if !self.is_hidden {
            for item in &self.items {
                match &*item.borrow() {
                    LayerItem::Group(group) => group.borrow().draw_rendered(d),
                    LayerItem::Path(path) => path.borrow().draw(d, self.color),
                    LayerItem::Bitmap(bitmap) => bitmap.draw(d),
                }
            }
        }
    }

    pub fn draw_selected(&self, d: &mut impl RaylibDraw) {
        if !self.is_hidden {
            for item in &self.items {
                match &*item.borrow() {
                    LayerItem::Group(group) => group.borrow().draw_selected(d),
                    LayerItem::Path(path) => path.borrow().draw(d, self.color),
                    LayerItem::Bitmap(bitmap) => bitmap.draw(d),
                }
            }
        }
    }

    pub const GAP: f32 = 2.0;
    pub const INDENT: f32 = 6.0;
    pub const THUMBNAIL_SIZE: f32 = 32.0;
    pub const THUMBNAIL_INSET: f32 = 6.0;
    pub const LAYER_HEIGHT: f32 = 2.0 * Self::THUMBNAIL_INSET + Self::THUMBNAIL_SIZE;
    pub const LAYER_COLOR_WIDTH: f32 = 4.0;
    pub const TEXT_FONT_SIZE: f32 = 10.0;
    pub const EXPAND_COLLAPSE_SIZE: f32 = 10.0;

    /// Calls the closure on each visible item in the layer panel tree until `f` returns [`Some`], then returns the output of `f`.
    /// Returns [`None`] if `f` never returns [`Some`].
    pub(in crate::document) fn for_each_layer_tree_item_internal<T>(
        &mut self,
        panel: &Rectangle,
        y: &mut f32,
        x: f32,
        width: f32,
        f: &mut impl FnMut(LayerPanelTreeItem<'_>) -> Option<T>
    ) -> Option<T> {
        {
            let rec = Rectangle::new(x, *y, width, Self::LAYER_HEIGHT);
            if rec.check_collision_recs(panel) {
                let data = LayerPanelTreeItem::new(rec, LayerPanelTreeItemEx::Layer { layer: self });

                let result = f(data);
                if result.is_some() { return result; }
            }

            *y += Self::LAYER_HEIGHT + Self::GAP;
        }

        if self.is_expanded {
            for item in self.items.iter().rev() {
                let item = &mut *item.borrow_mut();
                match item {
                    LayerItem::Group(group) => {
                        let result = group.borrow_mut().for_each_layer_tree_item_internal(panel, y, x + Self::INDENT, width - Self::INDENT, f);
                        if result.is_some() { return result; }
                    }

                    LayerItem::Path(path) => {
                        let rec = Rectangle::new(x + Self::INDENT, *y, width - Self::INDENT, Self::LAYER_HEIGHT);
                        if rec.check_collision_recs(panel) {
                            let data = LayerPanelTreeItem::new(rec, LayerPanelTreeItemEx::Path { layer: &self, path });
                            let result = f(data);
                            if result.is_some() { return result }
                        }
                        *y += Self::LAYER_HEIGHT + Self::GAP;
                    }

                    LayerItem::Bitmap(bitmap) => {
                        let rec = Rectangle::new(x + Self::INDENT, *y, width - Self::INDENT, Self::LAYER_HEIGHT);
                        if rec.check_collision_recs(panel) {
                            let data = LayerPanelTreeItem::new(rec, LayerPanelTreeItemEx::Bitmap { layer: &self, bitmap });
                            let result = f(data);
                            if result.is_some() { return result }
                        }
                        *y += Self::LAYER_HEIGHT + Self::GAP;
                    }
                }
            }
        }

        None
    }
}
