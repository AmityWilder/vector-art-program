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

pub enum LayerPanelTreeItemData<'a> {
    Layer {
        slot: Rectangle,
        color_rec: Rectangle,
        thumbnail_rec: Rectangle,
        name_rec: Rectangle,
        expand_collapse_rec: Rectangle,
        layer: &'a mut Layer,
    },
    Path {
        slot: Rectangle,
        thumbnail_rec: Rectangle,
        path: &'a mut Rc<RefCell<VectorPath>>,
    },
    Bitmap {
        slot: Rectangle,
        thumbnail_rec: Rectangle,
        bitmap: &'a mut Bitmap,
    },
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

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        if !self.is_hidden {
            for item in &self.items {
                match &*item.borrow() {
                    LayerItem::Group(group) => group.borrow().draw(d),
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

    /// Calls the closure on each visible item in the layer panel tree until `f` returns [`Some`], then returns the output of `f`.
    /// Returns [`None`] if `f` never returns [`Some`].
    pub(in crate::document) fn for_each_layer_tree_item_internal<T>(
        &mut self,
        panel: &Rectangle,
        y: &mut f32,
        x: f32,
        width: f32,
        f: &mut impl FnMut(LayerPanelTreeItemData<'_>) -> Option<T>
    ) -> Option<T> {
        {
            let mut rec = Rectangle::new(x, *y, width, Self::LAYER_HEIGHT);
            if rec.check_collision_recs(panel) {
                let data = LayerPanelTreeItemData::Layer {
                    slot: rec,
                    color_rec: {
                        rec.width = Self::LAYER_COLOR_WIDTH;
                        rec
                    },
                    thumbnail_rec: {
                        rec.x += Self::LAYER_COLOR_WIDTH + Self::THUMBNAIL_INSET;
                        rec.y += Self::THUMBNAIL_INSET;
                        (rec.width, rec.height) = (Self::THUMBNAIL_SIZE, Self::THUMBNAIL_SIZE);
                        rec
                    },
                    name_rec: {
                        rec.x += Self::THUMBNAIL_SIZE + Self::THUMBNAIL_INSET;
                        rec.height = 10.0;
                        rec.width = width - Self::LAYER_COLOR_WIDTH + Self::THUMBNAIL_INSET - Self::THUMBNAIL_SIZE;
                        rec
                    },
                    expand_collapse_rec: {
                        rec.y += 12.0;
                        rec.width = 10.0;
                        rec.height = 10.0;
                        rec
                    },
                    layer: self,
                };

                let result = f(data);
                if result.is_some() {
                    return result;
                }
            }

            *y += Self::LAYER_HEIGHT + Self::GAP;
        }

        if self.is_expanded {
            for item in self.items.iter().rev() {
                let item = &mut *item.borrow_mut();
                match item {
                    LayerItem::Group(group) => {
                        let result = group.borrow_mut().for_each_layer_tree_item_internal(panel, y, x + Self::INDENT, width - Self::INDENT, f);
                        if result.is_some() {
                            return result;
                        }
                    }

                    LayerItem::Path(path) => {
                        let mut rec = Rectangle::new(x + Self::INDENT, *y, width - Self::INDENT, Self::LAYER_HEIGHT);
                        if rec.check_collision_recs(panel) {
                            f(LayerPanelTreeItemData::Path {
                                slot: rec,
                                thumbnail_rec: {
                                    rec.x += Self::LAYER_COLOR_WIDTH + Self::THUMBNAIL_INSET;
                                    rec.y += Self::THUMBNAIL_INSET;
                                    (rec.width, rec.height) = (Self::THUMBNAIL_SIZE, Self::THUMBNAIL_SIZE);
                                    rec
                                },
                                path,
                            });
                        }
                        *y += Self::LAYER_HEIGHT + Self::GAP;
                    }

                    LayerItem::Bitmap(bitmap) => {
                        let mut rec = Rectangle::new(x + Self::INDENT, *y, width - Self::INDENT, Self::LAYER_HEIGHT);
                        if rec.check_collision_recs(panel) {
                            f(LayerPanelTreeItemData::Bitmap {
                                slot: rec,
                                thumbnail_rec: {
                                    rec.x += Self::LAYER_COLOR_WIDTH + Self::THUMBNAIL_INSET;
                                    rec.y += Self::THUMBNAIL_INSET;
                                    (rec.width, rec.height) = (Self::THUMBNAIL_SIZE, Self::THUMBNAIL_SIZE);
                                    rec
                                },
                                bitmap,
                            });
                        }
                        *y += Self::LAYER_HEIGHT + Self::GAP;
                    }
                }
            }
        }

        None
    }
}
