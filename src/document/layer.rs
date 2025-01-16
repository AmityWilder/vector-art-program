use std::{cell::RefCell, rc::Rc};
use raylib::prelude::*;
use crate::{appearance::Blending, Bitmap, VectorPath};

pub const INSET: f32 = 2.0;
pub const GAP: f32 = 2.0;
pub const INDENT: f32 = 6.0;
pub const THUMBNAIL_SIZE: f32 = 32.0;
pub const THUMBNAIL_INSET: f32 = 6.0;
pub const LAYER_HEIGHT: f32 = 2.0 * THUMBNAIL_INSET + THUMBNAIL_SIZE;
pub const LAYER_COLOR_WIDTH: f32 = 4.0;
pub const TEXT_FONT_SIZE: f32 = 10.0;
pub const EXPAND_COLLAPSE_SIZE: f32 = 10.0;

pub struct Group {
    pub group: Vec<Rc<RefCell<Layer>>>,
    pub is_expanded: bool,
    pub expand_button_rec: Rectangle,
}

pub enum LayerContent {
    Group(Group),
    Path(Rc<RefCell<VectorPath>>),
    Bitmap(Rc<RefCell<Bitmap>>),
}

pub struct Layer {
    pub slot_rec: Rectangle,
    pub thumbnail_rec: Rectangle,
    /// Name of the layer in the layers panel
    pub name: String,
    pub name_rec: Rectangle,
    /// Color of paths
    pub color: Color,
    pub color_rec: Rectangle,
    /// Skip in rendering
    pub is_hidden: bool,
    pub hide_button_rec: Rectangle,
    /// Disallow selection and editing
    pub is_locked: bool,
    pub lock_button_rec: Rectangle,
    /// Items move with layer
    pub is_group: bool,
    pub blend: Blending,
    pub artwork_bounds: Rectangle,
    pub content: LayerContent,
}

impl Layer {
    pub fn new(name: String, color: Color, content: LayerContent) -> Self {
        Self {
            slot_rec: Rectangle::default(),
            thumbnail_rec: Rectangle::default(),
            name,
            name_rec: Rectangle::default(),
            color,
            color_rec: Rectangle::default(),
            is_hidden: false,
            hide_button_rec: Rectangle::default(),
            is_locked: false,
            lock_button_rec: Rectangle::default(),
            is_group: false,
            blend: Blending::default(),
            artwork_bounds: Rectangle::default(),
            content,
        }
    }

    pub fn update_ui_recs(&mut self, container: &Rectangle, top: f32, indent: usize) {
        let indent_size = indent as f32 * INDENT;
        let left = container.x + indent_size;
        let width = container.width - indent_size;
        let mut rec = Rectangle::new(left, top, width, LAYER_HEIGHT);
        self.slot_rec = rec;
        self.color_rec = {
            rec.width = LAYER_COLOR_WIDTH;
            rec
        };
        self.thumbnail_rec = {
            rec.x += LAYER_COLOR_WIDTH + THUMBNAIL_INSET;
            rec.y += THUMBNAIL_INSET;
            (rec.width, rec.height) = (THUMBNAIL_SIZE, THUMBNAIL_SIZE);
            rec
        };
        self.name_rec = {
            rec.x += THUMBNAIL_SIZE + THUMBNAIL_INSET;
            rec.height = TEXT_FONT_SIZE;
            rec.width = width - LAYER_COLOR_WIDTH + THUMBNAIL_INSET - THUMBNAIL_SIZE;
            rec
        };
        match &mut self.content {
            LayerContent::Group(Group { expand_button_rec, .. }) => {
                *expand_button_rec = {
                    rec.y += TEXT_FONT_SIZE + 2.0;
                    rec.width  = EXPAND_COLLAPSE_SIZE;
                    rec.height = EXPAND_COLLAPSE_SIZE;
                    rec
                };
            }
            _ => (),
        }
    }

    pub fn draw_rendered(&self, d: &mut impl RaylibDraw) {
        if !self.is_hidden {
            match &self.content {
                LayerContent::Group(Group { group, .. }) => for layer in group { layer.borrow().draw_rendered(d) },
                LayerContent::Path(path) => path.borrow().draw(d, self.color),
                LayerContent::Bitmap(bitmap) => bitmap.borrow().draw(d),
            }
        }
    }

    pub fn draw_selected(&self, d: &mut impl RaylibDraw) {
        if !self.is_hidden {
            match &self.content {
                LayerContent::Group(Group { group, .. }) => for layer in group { layer.borrow().draw_selected(d) },
                LayerContent::Path(path) => path.borrow().draw(d, self.color),
                LayerContent::Bitmap(bitmap) => bitmap.borrow().draw(d),
            }
        }
    }
}
