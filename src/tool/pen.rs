use raylib::prelude::*;
use amymath::prelude::*;
use amylib::{iter::directed::DirectibleDoubleEndedIterator, rc::*};
use crate::{layer::{BackToFore, ForeToBack, LayerData, LayerType}, vector_path::{path_point::{Ctrl, CtrlPt1, CtrlPt2, PathPoint}, VectorPath}, Change, Document};
use super::{point_selection::HOVER_RADIUS_SQR, ToolType};

struct AddPointAction {
    target: StrongMut<VectorPath>,
    side: Ctrl,
    pp: PathPoint,
}

impl Change for AddPointAction {
    fn redo(&mut self, _document: &mut Document) -> Result<(), String> {
        let mut path = self.target.write();
        let pp = self.pp.clone();
        match self.side {
            Ctrl::In  => path.points.push_front(pp),
            Ctrl::Out => path.points.push_back (pp),
        }
        Ok(())
    }

    fn undo(&mut self, _document: &mut Document) -> Result<(), String> {
        let mut path = self.target.write();
        match self.side {
            Ctrl::In  => _ = path.points.pop_front(),
            Ctrl::Out => _ = path.points.pop_back (),
        }
        Ok(())
    }
}

/// The pen tool, possibly waiting for a target
pub enum Pen {
    Inactive(Option<StrongMut<VectorPath>>),
    Active {
        /// If [`Some`], continue seleted.
        /// If [`None`], find a hovered path or create a new path upon clicking.
        /// Must be a `VectorPath` layer.
        /// If there is a layer, it must not die before the pen dies.
        target: StrongMut<VectorPath>,

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
        Self::Inactive(None)
    }

    fn find_target(document: &mut Document, mouse_world_pos: Vector2) -> Self {
        // starting a new path
        for layer in document.layers.dfs_iter_mut(|_| false).cdir::<ForeToBack>() {
            // find hovered endpoint
            if let LayerData::Path(target) = &layer.data {
                let path = target.read();
                if let Some(last_idx) = path.points.len().checked_sub(1) { // failure to subtract 1 implies an empty list
                    let search_options = [(0, &path.points[0]), (last_idx, &path.points[last_idx])]; // heap allocations are yucky, ew. all my homies use stack arrays
                    let search_in = if last_idx != 0 { &search_options } else { &search_options[..=0] }; // only check last if the first isn't the last
                    for (idx, pp) in search_in {
                        if pp.p.distance_sqr_to(mouse_world_pos) <= HOVER_RADIUS_SQR {
                            return Self::Active {
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
        Self::Active {
            target: document.create_path(None, None).clone_mut(),
            is_dragging: false,
            direction: Ctrl::Out,
        }
    }
}

impl ToolType for Pen {
    fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            match self {
                Pen::Active { .. } => (),
                Pen::Inactive(Some(target)) => *self = Self::Active {
                    target: target.clone_mut(),
                    is_dragging: false,
                    direction: Ctrl::Out,
                },
                Pen::Inactive(None) => *self = Self::find_target(document, mouse_world_pos),
            }
        }

        if let Self::Active { target, is_dragging, direction  } = self {
            let mut path = target.write();

            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                // already drawing
                if let Some(opp_end) = match direction {
                    Ctrl::In  => path.points.back(),
                    Ctrl::Out => path.points.front(),
                } {
                    // close path
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
                let is_closed = path.is_closed;
                let pp = match direction {
                    Ctrl::In  => path.points.front(),
                    Ctrl::Out => path.points.back(),
                }.cloned().expect("point should have been created to have been dragging it");
                drop(path);
                document.push_change(Box::new(AddPointAction {
                    target: target.clone_mut(),
                    side: *direction,
                    pp,
                }));
                if is_closed {
                    *self = Self::Inactive(None);
                }
            }
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, document: &Document, mouse_world_pos: Vector2) {
        let px_world_size = document.camera.zoom.recip();
        match self {
            Self::Active { target, .. } | Self::Inactive(Some(target)) => {
                let path = target.read();
                path.draw_selected(d, px_world_size);
                let color = path.settings.read().color;

                if let Self::Active { direction, .. } = self {
                    match direction {
                        Ctrl::In => {
                            let mut iter = path.points.iter();
                            if let Some(pp_latest) = iter.next() {
                                pp_latest.draw(d, px_world_size, color, true, true, true);
                                for pp in iter {
                                    pp.draw(d, px_world_size, color, false, false, false);
                                }
                            }
                        }
                        Ctrl::Out => {
                            let mut iter = path.points.iter().rev();
                            if let Some(pp_latest) = iter.next() {
                                pp_latest.draw(d, px_world_size, color, true, true, true);
                                for pp in iter {
                                    pp.draw(d, px_world_size, color, false, false, false);
                                }
                            }
                        }
                    }
                }
            }
            Self::Inactive(None) => {
                // show selectable
                for layer in document.layers.shallow_iter().cdir::<BackToFore>() {
                    if let LayerData::Path(path) = &layer.data {
                        let path = path.read();
                        let color = path.settings.read().color;
                        if path.points.iter().any(|pp| pp.p.distance_sqr_to(mouse_world_pos) <= HOVER_RADIUS_SQR) {
                            path.draw_selected(d, px_world_size);
                        }
                        if let Some(last_idx) = path.points.len().checked_sub(1) {
                            let pp = &path.points[last_idx];
                            pp.draw(d, px_world_size, color, false, false, false);
                            if path.points.len() > 1 {
                                let pp = &path.points[0];
                                pp.draw(d, px_world_size, color, false, false, false);
                            }
                        }
                    }
                }
            }
        }
    }
}
