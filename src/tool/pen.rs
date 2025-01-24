use raylib::prelude::*;
use crate::{layer::{rc::{StrongMut, StrongRef}, tree::TreeIterDir, Layer, LayerType}, vector_path::path_point::{Ctrl, CtrlPt1, CtrlPt2, DistanceSqr, PathPoint}, Document};
use super::{direct_selection::HOVER_RADIUS_SQR, ToolType};

/// The pen tool, possibly waiting for a target
pub enum Pen {
    Inactive,
    Active {
        /// If [`Some`], continue seleted.
        /// If [`None`], find a hovered path or create a new path upon clicking.
        /// Must be a `VectorPath` layer.
        /// If there is a layer, it must not die before the pen dies.
        target: StrongMut<Layer>,

        /// Whether we are modifying an existing point or creating a new one
        is_dragging: bool,

        /// Whether points are being pushed to the front instead of the back \
        /// [`Ctrl::Out`] -> push to back \
        /// [`Ctrl::In`] -> push to front
        direction: Ctrl,
    },
}

impl Pen {
    pub fn new() -> Self {
        Self::Inactive
    }

    pub fn with_target(target: StrongMut<Layer>) -> Self {
        assert!(matches!(&*target.read(), Layer::Path(_)), "`target` is required to be a vector path");
        Self::Active {
            target,
            is_dragging: false,
            direction: Ctrl::Out,
        }
    }

    fn find_target(document: &mut Document, mouse_world_pos: Vector2) -> Self {
        // starting a new path
        for (layer, _) in document.layers.tree_iter_mut(TreeIterDir::ForeToBack, |_| false) {
            // find hovered endpoint
            if let Layer::Path(path) = &*layer.read() {
                if let Some(last_idx) = path.points.len().checked_sub(1) { // failure to subtract 1 implies an empty list
                    let search_options = [(0, &path.points[0]), (last_idx, &path.points[last_idx])]; // heap allocations are yucky, ew. all my homies use stack arrays
                    let search_in = if last_idx != 0 { &search_options } else { &search_options[..=0] }; // only check last if the first isn't the last
                    for (idx, pp) in search_in {
                        if pp.p.distance_sqr_to(mouse_world_pos) <= HOVER_RADIUS_SQR {
                            return Self::Active {
                                target: layer.clone_mut(),
                                is_dragging: true,
                                direction: if *idx == 0 { Ctrl::In } else { Ctrl::Out },
                            };
                        }
                    }
                }
            }
        }

        // no luck? create a new path
        let new_path = document.create_path(None, None);
        Self::Active {
            target: new_path,
            is_dragging: false,
            direction: Ctrl::Out,
        }
    }
}

impl ToolType for Pen {
    fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2, _mouse_world_delta: Vector2) {
        if matches!(self, Self::Inactive) && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            *self = Self::find_target(document, mouse_world_pos);
        }

        if let Self::Active { target, is_dragging, direction  } = self {
            let mut target = target.write();
            let Layer::Path(path) = &mut *target else { panic!("`target` is required to be a vector path") };

            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                // already drawing
                let opp_end = match direction {
                    Ctrl::In  => path.points.back(),
                    Ctrl::Out => path.points.front(),
                };
                if let Some(opp_end) = opp_end {
                    if opp_end.p.distance_sqr_to(mouse_world_pos) <= HOVER_RADIUS_SQR {
                        path.is_closed = true;
                        *is_dragging = true;
                    }
                }
            }

            if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                if *is_dragging {
                    // modifying an existing path point
                    let pp = &mut match direction {
                        Ctrl::In  => path.points.front_mut(),
                        Ctrl::Out => path.points.back_mut(),
                    }.expect("shouldn't have been able to select a path that had no points originally");

                    if let Some(CtrlPt1 { c1: (c1_side, c1), c2 }) = pp.ctrls.as_mut() {
                        // modifying existing controls
                        if c1_side == direction {
                            *c1 = mouse_world_pos;
                        } else {
                            *c2 = Some(CtrlPt2::Exact(mouse_world_pos));
                        }
                    } else {
                        // no existing controls
                        pp.ctrls = Some(CtrlPt1 { c1: (*direction, mouse_world_pos), c2: Some(CtrlPt2::Smooth) });
                    }
                } else {
                    // creating a new point
                    let pp = PathPoint {
                        p: mouse_world_pos,
                        ctrls: None,
                    };
                    match direction {
                        Ctrl::Out => {
                            path.points.push_back(pp);
                            *is_dragging = true;
                        }
                        Ctrl::In => {
                            path.points.push_front(pp);
                            *is_dragging = true;
                        }
                    }
                }
            }

            if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                *is_dragging = false;
                if path.is_closed {
                    drop(target);
                    *self = Self::Inactive;
                }
            }
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, document: &Document, mouse_world_pos: Vector2) {
        let zoom_inv = document.camera.zoom.recip();
        match self {
            Self::Active { target, .. } => {
                let target = target.read();
                if let Layer::Path(path) = &*target {
                    path.draw_selected(d, &document.camera, zoom_inv);
                }
            }
            Self::Inactive => {
                // show selectable
                for (layer, _) in document.layers.tree_iter(TreeIterDir::BackToFore, |_| false) {
                    if let Layer::Path(path) = &*layer.read() {
                        if path.points.iter().any(|pp| pp.p.distance_sqr_to(mouse_world_pos) <= HOVER_RADIUS_SQR) {
                            path.draw_selected(d, &document.camera, zoom_inv);
                        }
                    }
                }
            }
        }
    }
}
