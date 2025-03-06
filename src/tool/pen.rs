use amyvec::curve::WidthProfile;
use raylib::prelude::*;
use amymath::prelude::{*, Vector2};
use amylib::iter::directed::DirectibleDoubleEndedIterator;
use crate::{appearance::{Appearance, StyleItem}, document::Document, editor::Editor, layer::{BackToFore, ForeToBack, Layer, LayerType}, shaders::ShaderTable, vector_path::{path_point::{Ctrl, Ctrl1, Ctrl2, PathPoint}, stroke, DrawPathPoint, VectorPath}};
use super::{point_selection::HOVER_RADIUS_SQR, ToolType};

pub struct InactivePen<'a>(pub(super) Option<&'a mut VectorPath>);
pub struct InactivePenImm<'a>(pub(super) Option<&'a VectorPath>);

pub struct ActivePen<'a> {
    /// If [`Some`], continue seleted.
    /// If [`None`], find a hovered path or create a new path upon clicking.
    /// Must be a `VectorPath` layer.
    /// If there is a layer, it must not die before the pen dies.
    pub(super) target: &'a mut VectorPath,

    /// Whether we are modifying an existing point or creating a new one
    is_dragging: bool,

    /// Whether points are being pushed to the front instead of the back \
    /// [`Ctrl::Out`] -> push to back \
    /// [`Ctrl::In`] -> push to front
    direction: Ctrl,
}
pub struct ActivePenImm<'a> {
    pub(super) target: &'a VectorPath,
    is_dragging: bool,
    direction: Ctrl,
}

enum ActivePenTickResult {
    /// Not finished
    StillActive,
    /// Finished, remember target.
    FinishedKeep,
    /// Finished, forget target.
    FinishedDrop,
}

impl<'a> ActivePen<'a> {
    fn tick(&mut self, rl: &mut RaylibHandle, mouse_world_pos: Vector2) -> ActivePenTickResult {
        let path = &mut *self.target;

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            // already drawing
            if let Some(opp_end) = match self.direction {
                Ctrl::In  => path.curve.points.back(),
                Ctrl::Out => path.curve.points.front(),
            } {
                // close path
                if path.curve.points.len() > 1 && opp_end.p.distance_sqr(mouse_world_pos) <= HOVER_RADIUS_SQR {
                    path.curve.is_closed = true;
                    for profile in path.appearance.items.iter_mut().filter_map(|item|
                        if let StyleItem::Stroke(stroke::Stroke { thick: WidthProfile::Variable(profile), .. }) = item { Some(profile) } else { None }
                    ) {
                        profile.is_closed = true;
                    }
                    drop(path);
                    return ActivePenTickResult::FinishedKeep;
                }
            }
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            if self.is_dragging {
                // modifying an existing path point
                let pp = match self.direction {
                    Ctrl::In  => path.curve.points.front_mut(),
                    Ctrl::Out => path.curve.points.back_mut(),
                }.expect("shouldn't have been able to select a path that had no points originally");

                if let Some(Ctrl1 { c1: (c1_side, c1), c2 }) = pp.c.as_mut() {
                    // modifying existing controls
                    if c1_side == &self.direction {
                        *c1 = mouse_world_pos;
                    } else {
                        *c2 = Some(Ctrl2::Exact(mouse_world_pos));
                    }
                } else {
                    // no existing controls
                    pp.c = Some(Ctrl1 { c1: (self.direction, mouse_world_pos), c2: Some(Ctrl2::Reflect) });
                }
            } else {
                // creating a new point
                let pp = PathPoint {
                    p: mouse_world_pos,
                    c: None,
                };
                match self.direction {
                    Ctrl::Out => {
                        path.curve.points.push_back(pp);
                        self.is_dragging = true;
                    }
                    Ctrl::In => {
                        path.curve.points.push_front(pp);
                        self.is_dragging = true;
                    }
                }
            }
        }

        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            self.is_dragging = false;
            let is_closed = path.curve.is_closed;
            // todo: rework undo/redo

            // let pp = match self.direction {
            //     Ctrl::In  => path.curve.points.front(),
            //     Ctrl::Out => path.curve.points.back(),
            // }.copied().expect("point should have been created to have been dragging it");

            // document.push_change(Box::new(AddPointAction {
            //     target: self.target.clone_mut(),
            //     side: self.direction,
            //     pp,
            // }));
            if is_closed {
                return ActivePenTickResult::FinishedDrop;
            }
        }

        ActivePenTickResult::StillActive
    }
}

/// The pen tool, possibly waiting for a target
pub enum Pen<'a> {
    Inactive(InactivePen<'a>),
    Active(ActivePen<'a>),
}
pub enum PenImm<'a> {
    Inactive(InactivePenImm<'a>),
    Active(ActivePenImm<'a>),
}

impl Default for Pen<'_> {
    fn default() -> Self {
        Self::Inactive(InactivePen(None))
    }
}

impl<'a> Pen<'a> {
    pub fn new() -> Self {
        Self::Inactive(InactivePen(None))
    }

    pub fn target(&self) -> Option<&VectorPath> {
        if let Pen::Inactive(InactivePen(Some(target))) | Pen::Active(ActivePen { target, .. }) = self {
            return Some(&**target);
        }
        None
    }

    pub fn target_mut(&mut self) -> Option<&mut VectorPath> {
        if let Pen::Inactive(InactivePen(Some(target))) | Pen::Active(ActivePen { target, .. }) = self {
            return Some(&mut **target);
        }
        None
    }

    fn find_target<'b: 'a>(current_appearance: &Appearance, document: &'b mut Document, mouse_world_pos: Vector2) -> ActivePen<'a> {
        // starting a new path

        // find hovered endpoint
        for layer in document.layers.dfs_iter_mut(|_| false).cdir::<ForeToBack>() {
            if let Layer::Path(target) = layer {
                let iter = target.curve.points.front().map(|pp| (Ctrl::In,  pp)).into_iter()
                    .chain(target.curve.points.back ().map(|pp| (Ctrl::Out, pp)).into_iter());

                for (direction, pp) in iter {
                    if pp.p.distance_sqr(mouse_world_pos) <= HOVER_RADIUS_SQR {
                        drop(pp);
                        return ActivePen {
                            target,
                            is_dragging: true,
                            direction,
                        };
                    }
                }
            }
        }

        // no luck? create a new path
        document.create_path(None, None, current_appearance.clone());
        let Some(Layer::Path(target)) = document.layers.last_mut() else { panic!("should have just pushed a path") };
        ActivePen {
            target,
            is_dragging: false,
            direction: Ctrl::Out,
        }
    }
}

impl<'a> ToolType<'a> for Pen<'a> {
    fn tick<'b: 'a>(
        &mut self,
        rl: &mut RaylibHandle,
        _thread: &RaylibThread,
        current_appearance: &mut Appearance,
        document: &'b mut Document,
        _scratch_rtex: &mut Vec<RenderTexture2D>,
        mouse_world_pos: Vector2,
        _px_world_size: f32,
    ) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if matches!(self, Self::Inactive(_)) {
                let temp = std::mem::take(self);
                let Self::Inactive(InactivePen(temp)) = temp else { unreachable!("guarded by if condition") };
                *self = Self::Active(
                    if let Some(target) = temp {
                        ActivePen {
                            target,
                            is_dragging: false,
                            direction: Ctrl::Out,
                        }
                    } else {
                        Self::find_target(current_appearance, document, mouse_world_pos)
                    }
                );
            }
        }

        if let Self::Active(pen) = self {
            match pen.tick(rl, mouse_world_pos) {
                ActivePenTickResult::StillActive => (),
                ActivePenTickResult::FinishedKeep => *self = Self::Inactive(InactivePen(None)),
                ActivePenTickResult::FinishedDrop => {
                    let temp = std::mem::take(self);
                    let Self::Active(ActivePen { target, .. }) = temp else { unreachable!("could not have gotten here if self was inactive") };
                    *self = Self::Inactive(InactivePen(Some(target)));
                }
            }
        }
    }

    fn draw(
        &self,
        d: &mut impl RaylibDraw,
        editor: &Editor,
        _shader_table: &ShaderTable,
        px_world_size: f32,
        viewport: &Rect2,
        #[cfg(dev)] _mouse_world_pos: Vector2,
    ) {
        let info = match self {
            Self::Active(ActivePen { target, direction, .. }) => Some((target, Some(direction))),
            Self::Inactive(InactivePen(Some(target))) => Some((target, None)),
            Self::Inactive(InactivePen(None)) => None,
        };

        if let Some((target, maybe_direction)) = info {
            let path = &**target;
            path.draw_selected(d, px_world_size);
            let color = path.settings.color;

            if let Some(direction) = maybe_direction {
                let mut iter = path.curve.points.iter();
                if let Some(pp_latest) = match direction { Ctrl::In => iter.next(), Ctrl::Out => iter.next_back(), } {
                    d.draw_path_point(pp_latest, px_world_size, color, true, true, true);
                }
                for pp in iter {
                    d.draw_path_point(pp, px_world_size, color, false, false, false);
                }
            }
        } else {
            // show selectable
            for layer in editor.document.layers.shallow_iter().cdir::<BackToFore>() {
                if let Layer::Path(path) = layer {
                    let path = &*path;
                    let color = path.settings.color;
                    // if path.curve.points.iter().any(|pp| pp.p.distance_sqr(mouse_world_pos) <= HOVER_RADIUS_SQR) {
                    //     path.draw_selected(d, px_world_size);
                    // }
                    if let Some(last_idx) = path.curve.points.len().checked_sub(1) {
                        let pp = &path.curve.points[last_idx];
                        if viewport.contains_v(&pp.p) {
                            d.draw_path_point(pp, px_world_size, color, false, false, false);
                        }
                        if path.curve.points.len() > 1 {
                            let pp = &path.curve.points[0];
                            if viewport.contains_v(&pp.p) {
                                d.draw_path_point(pp, px_world_size, color, false, false, false);
                            }
                        }
                    }
                }
            }
        }
    }
}
