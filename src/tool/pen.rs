use amyvec::curve::WidthProfile;
use raylib::prelude::*;
use amymath::prelude::{*, Vector2};
use amylib::{iter::directed::DirectibleDoubleEndedIterator, rc::prelude::*};
use crate::{appearance::{Appearance, StyleItem}, document::Document, editor::Editor, layer::{BackToFore, ForeToBack, Layer, LayerType}, shaders::ShaderTable, vector_path::{path_point::{Ctrl, Ctrl1, Ctrl2, PathPoint}, stroke, DrawPathPoint, VectorPath}};
use super::{point_selection::HOVER_RADIUS_SQR, ToolType};

pub struct InactivePen(pub(super) Option<StrongMut<VectorPath>>);

pub struct ActivePen {
    /// If [`Some`], continue seleted.
    /// If [`None`], find a hovered path or create a new path upon clicking.
    /// Must be a `VectorPath` layer.
    /// If there is a layer, it must not die before the pen dies.
    pub(super) target: StrongMut<VectorPath>,

    /// Whether we are modifying an existing point or creating a new one
    is_dragging: bool,

    /// Whether points are being pushed to the front instead of the back \
    /// [`Ctrl::Out`] -> push to back \
    /// [`Ctrl::In`] -> push to front
    direction: Ctrl,
}

impl ActivePen {
    fn tick(&mut self, rl: &mut RaylibHandle, mouse_world_pos: Vector2) -> Option<InactivePen> {
        let mut path = self.target.write();

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
                    return Some(InactivePen(Some(self.target.clone_mut())))
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
                return Some(InactivePen(None));
            }
        }

        None
    }
}

/// The pen tool, possibly waiting for a target
pub enum Pen {
    Inactive(InactivePen),
    Active(ActivePen),
}

impl Pen {
    pub fn new() -> Self {
        Self::Inactive(InactivePen(None))
    }

    pub fn target(&self) -> Option<Strong<VectorPath>> {
        if let Pen::Inactive(InactivePen(Some(target))) | Pen::Active(ActivePen { target, .. }) = self {
            return Some(target.clone_ref());
        }
        None
    }

    pub fn target_mut(&mut self) -> Option<StrongMut<VectorPath>> {
        if let Pen::Inactive(InactivePen(Some(target))) | Pen::Active(ActivePen { target, .. }) = self {
            return Some(target.clone_mut());
        }
        None
    }

    fn find_target(current_appearance: &Appearance, document: &mut Document, mouse_world_pos: Vector2) -> ActivePen {
        // starting a new path
        for layer in document.layers.dfs_iter_mut(|_| false).cdir::<ForeToBack>() {
            // find hovered endpoint
            if let Layer::Path(target) = layer {
                let path = target.read();
                if let Some(last_idx) = path.curve.points.len().checked_sub(1) { // failure to subtract 1 implies an empty list
                    let search_options = [(0, &path.curve.points[0]), (last_idx, &path.curve.points[last_idx])]; // heap allocations are yucky, ew. all my homies use stack arrays
                    let search_in = if last_idx != 0 { &search_options } else { &search_options[..=0] }; // only check last if the first isn't the last
                    for (idx, pp) in search_in {
                        if pp.p.distance_sqr(mouse_world_pos) <= HOVER_RADIUS_SQR {
                            return ActivePen {
                                target: target.clone_mut(),
                                is_dragging: true,
                                direction: if *idx == 0 { Ctrl::In } else { Ctrl::Out },
                            };
                        }
                    }
                }
            }
        }

        // no luck? create a new path
        ActivePen {
            target: document.create_path(None, None, current_appearance.clone()).clone_mut(),
            is_dragging: false,
            direction: Ctrl::Out,
        }
    }
}

impl ToolType for Pen {
    fn tick(
        &mut self,
        rl: &mut RaylibHandle,
        _thread: &RaylibThread,
        current_appearance: &mut Appearance,
        document: &mut Document,
        _scratch_rtex: &mut Vec<RenderTexture2D>,
        mouse_world_pos: Vector2,
        _px_world_size: f32,
    ) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            match self {
                Pen::Active(_) => (),
                Pen::Inactive(InactivePen(Some(target))) => *self = Self::Active(ActivePen {
                    target: target.clone_mut(),
                    is_dragging: false,
                    direction: Ctrl::Out,
                }),
                Pen::Inactive(InactivePen(None)) => *self = Self::Active(Self::find_target(current_appearance, document, mouse_world_pos)),
            }
        }

        if let Self::Active(pen) = self {
            if let Some(inactive_pen) = pen.tick(rl, mouse_world_pos) {
                *self = Self::Inactive(inactive_pen);
            }
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, editor: &Editor, _shader_table: &ShaderTable, px_world_size: f32, viewport: &Rect2, #[cfg(dev)] _mouse_world_pos: Vector2) {
        let info = match self {
            Self::Active(ActivePen { target, direction, .. }) => Some((target, Some(direction))),
            Self::Inactive(InactivePen(Some(target))) => Some((target, None)),
            Self::Inactive(InactivePen(None)) => None,
        };

        if let Some((target, maybe_direction)) = info {
            let path = target.read();
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
                    let path = path.read();
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
