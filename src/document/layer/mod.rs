use amylib::{collections::tree::*, iter::directed::*};
use raylib::prelude::*;
use crate::{appearance::Blending, raster::Raster, vector_path::VectorPath};

pub mod ui_iter;
pub mod group;

use group::Group;

pub struct LayerSettings {
    /// Name of the layer in the layers panel
    pub name: String,
    /// Color of paths
    pub color: Color,
    /// Skip in rendering
    pub is_hidden: bool,
    /// Disallow selection and editing
    pub is_locked: bool,
    /// Items move with layer
    pub is_group: bool,
    pub blend: Blending,
    pub artwork_bounds: Rectangle,
}

impl LayerSettings {
    pub fn new(name: String, color: Color) -> Self {
        Self {
            name,
            color,
            is_hidden: false,
            is_locked: false,
            is_group: false,
            blend: Blending::default(),
            artwork_bounds: Rectangle::default(),
        }
    }
}

pub enum Layer {
    Group(Group),
    Path(VectorPath),
    Raster(Raster),
}

pub trait LayerType {
    fn settings(&self) -> &LayerSettings;
    fn settings_mut(&mut self) -> &mut LayerSettings;

    /// Draw without helper visuals
    fn draw_rendered(&self, d: &mut impl RaylibDraw);

    /// Draw with helper visuals
    fn draw_selected(&self, d: &mut impl RaylibDraw, px_world_size: f32);
}

impl LayerType for Layer {
    fn settings(&self) -> &LayerSettings {
        match self {
            Layer::Group(group) => group.settings(),
            Layer::Path(path) => path.settings(),
            Layer::Raster(raster) => raster.settings(),
        }
    }

    fn settings_mut(&mut self) -> &mut LayerSettings {
        match self {
            Layer::Group(group) => group.settings_mut(),
            Layer::Path(path) => path.settings_mut(),
            Layer::Raster(raster) => raster.settings_mut(),
        }
    }

    fn draw_rendered(&self, d: &mut impl RaylibDraw) {
        if !self.settings().is_hidden {
            match self {
                Layer::Group(group) => group.draw_rendered(d),
                Layer::Path(path) => path.draw_rendered(d),
                Layer::Raster(raster) => raster.draw_rendered(d),
            }
        }
    }

    fn draw_selected(&self, d: &mut impl RaylibDraw, px_world_size: f32) {
        if !self.settings().is_hidden {
            match self {
                Layer::Group(group) => group.draw_selected(d, px_world_size),
                Layer::Path(path) => path.draw_selected(d, px_world_size),
                Layer::Raster(raster) => raster.draw_selected(d, px_world_size),
            }
        }
    }
}

pub type LayerTree = Tree<Layer>;

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

/// Start at the background and traverse to the foreground
///
/// Visit elements in the order they should be drawn so they occlude each other correctly
pub type BackToFore = CForward;

/// Start at the foreground and traverse to the background
///
/// Visit elements in the order that mouse collisions should find them
pub type ForeToBack = CReverse;

/// Start at the bottommost layer in the layer panel and traverse to the top
///
/// Reverse of `TopToBot` for sake of consistency - I haven't found a use for this yet
pub type BotToTop = CForward;

/// Start at the topmost layer in the layer panel and traverse to the bottom
///
/// Visit elements in the order their height influences following layers
pub type TopToBot = CReverse;
