use std::{ops, path::PathBuf};

use amylib::{collections::tree::{Recursive, Tree}, iter::directed::{CForward, CReverse}};
use raylib::prelude::*;
use crate::{raster::Raster, vector_path::VectorPath};

pub mod ui_iter;
pub mod group;

use group::Group;

#[derive(Default)]
pub struct LayerSettings {
    /// Name of the layer in the layers panel
    pub name: String,
    /// Color of paths when selected
    pub color: Color,
    /// Skip in rendering
    pub is_hidden: bool,
    /// Disallow selection and editing
    pub is_locked: bool,
    /// Items move with layer
    pub is_group: bool,
}

impl LayerSettings {
    pub fn new(name: String, color: Color) -> Self {
        Self {
            name,
            color,
            is_hidden: false,
            is_locked: false,
            is_group: false,
        }
    }
}

pub enum LayerSettingsRef<'a> {
    Group(&'a Group),
    Path(&'a VectorPath),
    Raster(&'a Raster),
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
    Path(&'a mut VectorPath),
    Raster(&'a mut Raster),
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
    Path(VectorPath),
    Raster(Raster),
}

impl Layer {
    pub fn settings(&self) -> &LayerSettings {
        match self {
            Layer::Group(group) => &group.settings,
            Layer::Path(path) => &path.settings,
            Layer::Raster(raster) => &raster.settings,
        }
    }

    pub fn settings_mut(&mut self) -> & mut LayerSettings {
        match self {
            Layer::Group(group) => &mut group.settings,
            Layer::Path(path) => &mut path.settings,
            Layer::Raster(raster) => &mut raster.settings,
        }
    }
}

pub struct LayerPath {
    inner: [u16],
}

impl LayerPath {
    pub fn new<S: AsRef<[u16]> + ?Sized>(s: &S) -> &LayerPath {
        unsafe { &*(s.as_ref() as *const [u16] as *const LayerPath) }
    }

    #[inline]
    pub fn as_slice(&self) -> &[u16] {
        &self.inner
    }

    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [u16] {
        &mut self.inner
    }
}

impl<'a> IntoIterator for &'a LayerPath {
    type Item = usize;
    type IntoIter = std::iter::Map<std::iter::Copied<std::slice::Iter<'a, u16>>, fn(u16) -> usize>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter().copied().map(move |n| n as usize)
    }
}

#[derive(Clone)]
pub struct LayerPathBuf {
    inner: Vec<u16>,
}

impl ops::Deref for LayerPathBuf {
    type Target = LayerPath;

    #[inline]
    fn deref(&self) -> &Self::Target {
        LayerPath::new(self.inner.as_slice())
    }
}

impl AsRef<LayerPath> for LayerPathBuf {
    #[inline]
    fn as_ref(&self) -> &LayerPath {
        self
    }
}

impl From<Box<LayerPath>> for LayerPathBuf {
    /// This conversion does not allocate or copy memory.
    #[inline]
    fn from(boxed: Box<LayerPath>) -> LayerPathBuf {
        let rw = Box::into_raw(boxed) as *mut [u16];
        let inner = unsafe { Box::from_raw(rw) };
        LayerPathBuf { inner: inner.to_vec() }
    }
}

impl From<LayerPathBuf> for Box<LayerPath> {
    /// This conversion currently should not allocate memory,
    /// but this behavior is not guaranteed on all platforms or in all future versions.
    #[inline]
    fn from(p: LayerPathBuf) -> Box<LayerPath> {
        let rw = Box::into_raw(p.inner.into_boxed_slice()) as *mut [u16];
        let rw = Box::into_raw(unsafe { Box::from_raw(rw) }) as *mut LayerPath;
        unsafe { Box::from_raw(rw) }
    }
}

impl LayerPathBuf {
    pub fn push(&mut self, value: u16) {
        self.inner.push(value);
    }
}

pub trait LayerType {
    /// Draw without helper visuals
    fn draw_rendered(&self, d: &mut impl RaylibDraw, camera: &Camera2D, scratch_rtex: &mut [RenderTexture2D]);

    /// Draw with helper visuals
    fn draw_selected(&self, d: &mut impl RaylibDraw, px_world_size: f32);
}

impl LayerType for Layer {
    fn draw_rendered(&self, d: &mut impl RaylibDraw, camera: &Camera2D, scratch_rtex: &mut [RenderTexture2D]) {
        if !self.settings().is_hidden {
            match self {
                Layer::Group(group) => group.draw_rendered(d, camera, scratch_rtex),
                Layer::Path(path) => path.draw_rendered(d, camera, scratch_rtex),
                Layer::Raster(raster) => raster.draw_rendered(d, camera, scratch_rtex),
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
