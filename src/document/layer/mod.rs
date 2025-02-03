use amylib::{collections::tree::{Recursive, Tree}, iter::directed::{CForward, CReverse}, rc::StrongMut};
use raylib::prelude::*;
use crate::{appearance::Blending, raster::Raster, vector_path::VectorPath};

pub mod ui_iter;
pub mod group;

use group::Group;

#[derive(Default)]
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
        }
    }
}

pub enum LayerSettingsRef<'a> {
    Group(&'a Group),
    Path(std::cell::Ref<'a, VectorPath>),
    Raster(std::cell::Ref<'a, Raster>),
}

impl std::ops::Deref for LayerSettingsRef<'_> {
    type Target = LayerSettings;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Group(group) => &group.settings,
            Self::Path(path) => &path.settings,
            Self::Raster(raster) => &raster.settings,
        }
    }
}

pub enum LayerSettingsRefMut<'a> {
    Group(&'a mut Group),
    Path(std::cell::RefMut<'a, VectorPath>),
    Raster(std::cell::RefMut<'a, Raster>),
}

impl std::ops::Deref for LayerSettingsRefMut<'_> {
    type Target = LayerSettings;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Group(group) => &group.settings,
            Self::Path(path) => &path.settings,
            Self::Raster(raster) => &raster.settings,
        }
    }
}

impl std::ops::DerefMut for LayerSettingsRefMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Group(group) => &mut group.settings,
            Self::Path(path) => &mut path.settings,
            Self::Raster(raster) => &mut raster.settings,
        }
    }
}

pub enum Layer {
    Group(Group),
    Path(StrongMut<VectorPath>),
    Raster(StrongMut<Raster>),
}

impl Layer {
    pub fn settings(&self) -> LayerSettingsRef<'_> {
        match self {
            Layer::Group(group) => LayerSettingsRef::Group(group),
            Layer::Path(path) => LayerSettingsRef::Path(path.read()),
            Layer::Raster(raster) => LayerSettingsRef::Raster(raster.read()),
        }
    }

    pub fn settings_mut(&mut self) -> LayerSettingsRefMut<'_> {
        match self {
            Layer::Group(group) => LayerSettingsRefMut::Group(group),
            Layer::Path(path) => LayerSettingsRefMut::Path(path.write()),
            Layer::Raster(raster) => LayerSettingsRefMut::Raster(raster.write()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test0() {
        let layer = Layer::Path(StrongMut::new(VectorPath::new(LayerSettings {
            name: "hello world".to_owned(),
            ..Default::default()
        })));
        let settings = layer.settings();
        let x = settings.name.as_str();
        assert_eq!(x, "hello world");
    }
}

pub trait LayerType {
    /// Draw without helper visuals
    fn draw_rendered(&self, d: &mut impl RaylibDraw);

    /// Draw with helper visuals
    fn draw_selected(&self, d: &mut impl RaylibDraw, px_world_size: f32);
}

impl LayerType for Layer {
    fn draw_rendered(&self, d: &mut impl RaylibDraw) {
        if !self.settings().is_hidden {
            match self {
                Layer::Group(group) => group.draw_rendered(d),
                Layer::Path(path) => path.read().draw_rendered(d),
                Layer::Raster(raster) => raster.read().draw_rendered(d),
            }
        }
    }

    fn draw_selected(&self, d: &mut impl RaylibDraw, px_world_size: f32) {
        if !self.settings().is_hidden {
            match self {
                Layer::Group(group) => group.draw_selected(d, px_world_size),
                Layer::Path(path) => path.read().draw_selected(d, px_world_size),
                Layer::Raster(raster) => raster.read().draw_selected(d, px_world_size),
            }
        }
    }
}

pub type LayerTree = Tree<Layer>;

impl Recursive for Layer {
    type Node = Group;
    #[inline]
    fn is_node(&self) -> bool {
        matches!(self, Layer::Group(_))
    }
    #[inline]
    fn if_node(&self) -> Option<&Self::Node> {
        match self {
            Layer::Group(g) => Some(g),
            _ => None,
        }
    }
    #[inline]
    fn if_node_mut(&mut self) -> Option<&mut Self::Node> {
        match self {
            Layer::Group(g) => Some(g),
            _ => None,
        }
    }
    #[inline]
    fn children(node: &Self::Node) -> &Tree<Self> {
        &node.items
    }
    #[inline]
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
