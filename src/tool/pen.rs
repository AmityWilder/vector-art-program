use raylib::prelude::*;
use crate::{layer::{rc::StrongLayerMut, tree::LayerIterDir, Layer, LayerType}, vector_path::path_point::{Ctrl, CtrlPt1, CtrlPt2, DistanceSqr, PathPoint}, Document};
use super::{direct_selection::HOVER_RADIUS_SQR, ToolType};

pub struct Pen {
    /// If [`Some`], continue seleted.
    /// If [`None`], find a hovered path or create a new path upon clicking.
    /// Must be a `VectorPath` layer.
    /// If there is a layer, it must not die before the pen dies.
    pub target: Option<StrongLayerMut>,

    /// [`Some`] while dragging, [`None`] otherwise.
    current_anchor: Option<usize>,

    /// Whether points are being pushed to the front instead of the back \
    /// [`Ctrl::Out`] -> push to back \
    /// [`Ctrl::In`] -> push to front
    direction: Ctrl,
}

impl Pen {
    pub fn new() -> Self {
        Self {
            target: None,
            current_anchor: None,
            direction: Ctrl::Out,
        }
    }
}

impl ToolType for Pen {
    fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2, _mouse_world_delta: Vector2) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if self.target.is_none() {
                // starting a new path
                for (layer, _) in document.layers.tree_iter_mut(LayerIterDir::ForeToBack, |_| false) {
                    // find hovered endpoint
                    if let Layer::Path(path) = &*layer.read() {
                        if let Some(last_idx) = path.points.len().checked_sub(1) { // failure to subtract 1 implies an empty list
                            let search_options = [(0, &path.points[0]), (last_idx, &path.points[last_idx])]; // heap allocations are yucky, ew. all my homies use stack arrays
                            let search_in = if last_idx != 0 { &search_options } else { &search_options[..=0] }; // only check last if the first isn't the last
                            for (idx, pp) in search_in {
                                if pp.p.distance_sqr_to(mouse_world_pos) <= HOVER_RADIUS_SQR {
                                    self.target = Some(layer.clone_mut());
                                    self.current_anchor = Some(*idx);
                                    self.direction = if *idx == 0 { Ctrl::In } else { Ctrl::Out };
                                    break;
                                }
                            }
                        }
                    }
                }

                // no luck?
                if self.target.is_none() {
                    // create a new path
                    let new_path = document.create_path(None, None);
                    self.target = Some(new_path);
                    self.current_anchor = None;
                    self.direction = Ctrl::Out;
                }
            }
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let mut target = self.target.as_mut().expect("`target` should have been set when mouse was pressed").write();
            let Layer::Path(path) = &mut *target else { panic!("`target` is required to be a vector path") };

            if let Some(idx) = self.current_anchor {
                // modifying an existing path point
                let pp = &mut path.points.get_mut(idx).expect("shouldn't have been able to select a path that had no points originally");
                if let Some(CtrlPt1 { c1: (c1_side, c1), c2 }) = pp.ctrls.as_mut() {
                    // modifying existing controls
                    if c1_side == &self.direction {
                        *c1 = mouse_world_pos;
                    } else {
                        *c2 = Some(CtrlPt2::Exact(mouse_world_pos));
                    }
                } else {
                    // no existing controls
                    pp.ctrls = Some(CtrlPt1 { c1: (self.direction, mouse_world_pos), c2: Some(CtrlPt2::Smooth) });
                }
            } else {
                // creating a new point
                let pp = PathPoint {
                    p: mouse_world_pos,
                    ctrls: None,
                };
                match self.direction {
                    Ctrl::Out => {
                        path.points.push_back(pp);
                        self.current_anchor = Some(path.points.len() - 1);
                    }
                    Ctrl::In => {
                        path.points.push_front(pp);
                        self.current_anchor = Some(0);
                    }
                }
            }
        }

        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            self.current_anchor = None;
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, document: &Document, mouse_world_pos: Vector2) {
        let zoom_inv = document.camera.zoom.recip();
        if let Some(target) = self.target.as_ref() {
            let target = target.read();
            if let Layer::Path(path) = &*target {
                path.draw_selected(d, &document.camera, zoom_inv);
            }
        } else {
            // show selectable
            for (layer, _) in document.layers.tree_iter(LayerIterDir::BackToFore, |_| false) {
                if let Layer::Path(path) = &*layer.read() {
                    if path.points.iter().any(|pp| pp.p.distance_sqr_to(mouse_world_pos) <= HOVER_RADIUS_SQR) {
                        path.draw_selected(d, &document.camera, zoom_inv);
                    }
                }
            }
        }
    }
}
