use amyvec::{curve::PathPointIdx, path_point::PathPoint};
use raylib::prelude::*;
use amymath::prelude::*;
use amylib::rc::prelude::*;
use crate::{layer::LayerType, shaders::ShaderTable, vector_path::{path_point::{Ctrl1, Ctrl2, PPPart}, VectorPath, DrawPathPoint}};
use super::{HOVER_RADIUS, SNAP_VERT_RADIUS, SNAP_VERT_RADIUS_SQR};

/// Allows manipulating velocity controls on one point
pub struct SingleSelect {
    pub target: StrongMut<VectorPath>,
    pub point: Option<PathPointIdx>,
}

impl SingleSelect {
    pub fn drag(&mut self, delta: Vector2) {
        let mut path = self.target.write();
        if let Some(idx) = self.point {
            let pp = &mut path.curve.points[idx.point];
            match &idx.part {
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

    pub fn end_dragging(&mut self, px_world_size: f32) {
        let mut path = self.target.write();
        if let Some(idx) = self.point {
            let pp = &mut path.curve.points[idx.point];
            let snap_vert_radius_sqr = SNAP_VERT_RADIUS_SQR * px_world_size * px_world_size;
            let p = pp.p;
            if let (PPPart::Ctrl(side), &mut Some(Ctrl1 { c1: (ref mut c1_side, ref mut c1), c2: ref mut c2 @ Some(Ctrl2::Exact(c2_pos)) })) = (idx.part, &mut pp.c) {

                let is_c_self_c1 = *c1_side == side;
                let (c_self, c_opp) = if is_c_self_c1 { (*c1, c2_pos) } else { (c2_pos, *c1) };

                let snapped = if c_self.distance_sqr_to(p) <= snap_vert_radius_sqr {
                    Some(None) // corner
                } else {
                    let refl = c_opp.reflected_over(p);
                    if c_self.distance_sqr_to(refl) <= snap_vert_radius_sqr {
                        Some(Some(Ctrl2::Reflect))
                    } else {
                        let len = (refl - p).normalized().dot(c_self - p);
                        let mirror = c_opp.reflected_to(p, len);
                        if c_self.distance_sqr_to(mirror) <= snap_vert_radius_sqr {
                            Some(Some(Ctrl2::Mirror(len)))
                        } else { None }
                    }
                };

                if let Some(snapped) = snapped {
                    if is_c_self_c1 && snapped.is_none() {
                        pp.c = None;
                    } else {
                        if is_c_self_c1 {
                            *c1_side = c1_side.opposite();
                            *c1 = c2_pos;
                        }
                        *c2 = snapped;
                    }
                }
            } else if let (PPPart::Ctrl(side), &mut Some(Ctrl1 { c1: (ref mut c1_side, ref mut c1), c2: None })) = (idx.part, &mut pp.c) {
                if *c1_side == side && c1.distance_sqr_to(p) <= snap_vert_radius_sqr {
                    pp.c = None;
                }
            }
        }
    }

    pub fn get_selected(&mut self, mouse_world_pos: Vector2, px_world_size: f32) -> Option<PathPointIdx> {
        let hover_radius = HOVER_RADIUS * px_world_size;
        let hover_radius_sqr = hover_radius * hover_radius;

        let path = self.target.read();
        let mut items = [None, None, None];
        if let Some(idx) = self.point {
            let pp = &path.curve.points[idx.point];
            let p_dist = pp.p.rec_distance_to(mouse_world_pos);
            items[0] = (p_dist <= hover_radius).then_some((PPPart::Anchor, p_dist));
            if let Some(Ctrl1 { c1: (c1_side, c1), c2 }) = pp.c {
                let c1_dist = c1.distance_sqr_to(mouse_world_pos);
                items[1] = (c1_dist <= hover_radius_sqr).then_some((PPPart::Ctrl(c1_side), c1_dist));
                if let Some(c2) = c2 {
                    let c2 = c2.calculate(pp.p, c1);
                    let c2_dist = c2.distance_sqr_to(mouse_world_pos);
                    let c2_side = c1_side.opposite();
                    items[2] = (c2_dist <= hover_radius_sqr).then_some((PPPart::Ctrl(c2_side), c2_dist));
                }
            }
            items.into_iter()
                .flatten()
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("distance should not be NaN"))
                .map(|(part, _)| PathPointIdx { point: idx.point, part })
        } else {
            None
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw, px_world_size: f32, selection_rec: Option<Rectangle>, _shader_table: &ShaderTable) {
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
            if let Some(idx) = self.point {
                path.draw_selected(d, px_world_size);
                let pp = &path.curve.points[idx.point];
                d.draw_path_point(pp, px_world_size, path.settings.color, true, true, true);
                if let (PPPart::Ctrl(side), &PathPoint { p, c: Some(Ctrl1 { c1: (c1_side, c1), c2: Some(Ctrl2::Exact(c2)) }) }) = (idx.part, pp) {
                    let (c_self, c_opp) = if c1_side == side { (c1, c2) } else { (c2, c1) };
                    let refl = c_opp.reflected_over(p);
                    let mirror = c_opp.reflected_to(p, (refl - p).normalized().dot(c_self - p));
                    d.draw_line_v(p, mirror, Color::new(255, 0, 255, 127));
                    d.draw_ring(refl, (SNAP_VERT_RADIUS - 0.5) * px_world_size, (SNAP_VERT_RADIUS + 0.5) * px_world_size, 0.0, 360.0, 16, Color::new(255, 0, 255, 127));
                    d.draw_ring(p, (SNAP_VERT_RADIUS - 0.5) * px_world_size, (SNAP_VERT_RADIUS + 0.5) * px_world_size, 0.0, 360.0, 16, Color::new(255, 0, 255, 127));
                }
            } else {
                path.draw_selected(d, px_world_size * 3.0);
            }
        }
    }
}
