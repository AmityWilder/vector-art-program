use std::fmt;
use raylib::prelude::*;
use amymath::prelude::*;
use amylib::rc::prelude::*;
use crate::{layer::LayerType, shaders::ShaderTable, vector_path::{path_point::{Ctrl1, Ctrl2, PPPart, PathPoint}, VectorPath, DrawPathPoint}, Change, Document};
use super::{HOVER_RADIUS, HOVER_RADIUS_SQR};

pub const SNAP_VERT_RADIUS: f32 = 5.0;
pub const SNAP_VERT_RADIUS_SQR: f32 = SNAP_VERT_RADIUS * SNAP_VERT_RADIUS;

struct EditSinglePointAction {
    target: StrongMut<VectorPath>,
    idx: Option<usize>,
    pre: PathPoint,
    post: PathPoint,
}

impl fmt::Debug for EditSinglePointAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EditSinglePointAction").finish()
    }
}

impl Change for EditSinglePointAction {
    fn redo(&mut self, _document: &mut Document) -> Result<(), String> {
        // let mut path = self.target.write();
        // path.curve.points[self.idx].clone_from(&self.post);
        Err("under construction".to_string())
    }

    fn undo(&mut self, _document: &mut Document) -> Result<(), String> {
        // let mut path = self.target.write();
        // path.curve.points[self.idx].clone_from(&self.pre);
        Err("under construction".to_string())
    }
}

/// Allows manipulating velocity controls on one point
pub struct SingleSelect {
    pub target: StrongMut<VectorPath>,
    pub pt_idx: Option<usize>,
    pub part: PPPart,
}

impl SingleSelect {
    pub fn drag(&mut self, delta: Vector2) {
        let mut path = self.target.write();
        if let Some(idx) = self.pt_idx {
            let pp = &mut path.curve.points[idx];
            match &self.part {
                PPPart::Anchor => {
                    pp.move_point(delta);
                }

                PPPart::Ctrl(side) => {
                    let Ctrl1 { c1: (ref c1_side, c1), c2 } = pp.c.as_mut().expect("should not select corner");
                    if c1_side == side {
                        *c1 += delta;
                    } else {
                        let c2 = c2.as_mut().expect("should not select corner");
                        match c2 {
                            Ctrl2::Exact(c2) => *c2 += delta,
                            _ => *c2 = Ctrl2::Exact(c2.calculate(pp.p, *c1) + delta),
                        }
                    }
                }
            }
        } else {
            for pp in &mut path.curve.points {
                pp.move_point(delta);
            }
        }
    }

    pub fn get_selected(&mut self, mouse_world_pos: Vector2) -> Option<PPPart> {
        let path = self.target.read();
        let mut items = [None, None, None];
        if let Some(idx) = self.pt_idx {
            let pp = &path.curve.points[idx];
            let p_dist = pp.p.rec_distance_to(mouse_world_pos);
            items[0] = (p_dist <= HOVER_RADIUS).then_some((PPPart::Anchor, p_dist));
            if let Some(Ctrl1 { c1: (c1_side, c1), c2 }) = pp.c {
                let c1_dist = c1.distance_sqr_to(mouse_world_pos);
                items[1] = (c1_dist <= HOVER_RADIUS_SQR).then_some((PPPart::Ctrl(c1_side), c1_dist));
                if let Some(c2) = c2 {
                    let c2 = c2.calculate(pp.p, c1);
                    let c2_dist = c2.distance_sqr_to(mouse_world_pos);
                    let c2_side = c1_side.opposite();
                    items[2] = (c2_dist <= HOVER_RADIUS_SQR).then_some((PPPart::Ctrl(c2_side), c2_dist));
                }
            }
        }
        items.into_iter()
            .flatten()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("distance should not be NaN"))
            .map(|(part, _)| part)
    }

    pub fn draw(&self, d: &mut impl RaylibDraw, _document: &Document, px_world_size: f32, selection_rec: Option<Rectangle>, _shader_table: &ShaderTable) {
        if let Some(_selection_rec) = selection_rec {
            unreachable!("2025-02-10: at present, creating a selection rectangle inherently clears the selection. this may change in the future with `shift+select`.");
            // use amylib::prelude::DirectibleDoubleEndedIterator;
            // use crate::{Layer, tool::point_selection::BackToFore};
            // // draw selection options
            // let selected_idx = self.pt_idx.expect("pt_idx should not be None when selecting with rectangle");
            // for (has_selected, path) in document.layers.shallow_iter().cdir::<BackToFore>().filter_map(|layer|
            //     if let Layer::Path(path) = layer {
            //         Some((&self.target == path, path.clone_ref()))
            //     } else { None })
            // {
            //     let path = path.read();
            //     path.draw_selected(d, px_world_size);
            //     for (is_point_selected, pp) in path.curve.points.iter().enumerate().map(|(i, pp)| (has_selected && i == selected_idx, pp)) {
            //         let is_selected = is_point_selected || selection_rec.check_collision_point_rec(pp.p);
            //         d.draw_path_point(pp, px_world_size, path.settings.color, is_selected, false, false);
            //     }
            // }
        } else {
            let path = self.target.read();
            if let Some(idx) = self.pt_idx {
                path.draw_selected(d, px_world_size);
                let pp = &path.curve.points[idx];
                d.draw_path_point(pp, px_world_size, path.settings.color, true, true, true);
                if let &PPPart::Ctrl(side) = &self.part {
                    let ctrls = pp.ctrl_pov(side);
                    if let Some(Ctrl2::Exact(c_opp)) = ctrls.1 {
                        let p = pp.p;
                        let c_self = ctrls.0.expect("should not select corner").calculate(p, c_opp);
                        let refl = c_opp.reflected_over(p);
                        let mirror = c_opp.reflected_to(p, (refl - p).normalized().dot(c_self - p));
                        d.draw_line_v(p, mirror, Color::new(255, 0, 255, 127));
                        d.draw_ring(refl, (SNAP_VERT_RADIUS - 0.5) * px_world_size, (SNAP_VERT_RADIUS + 0.5) * px_world_size, 0.0, 360.0, 16, Color::new(255, 0, 255, 127));
                        d.draw_rectangle_lines_rect2(Rect2::from_center_and_radius(p, SNAP_VERT_RADIUS * px_world_size), Color::new(255, 0, 255, 127));
                    }
                }
            } else {
                path.draw_selected(d, px_world_size * 3.0);
            }
        }
    }
}
