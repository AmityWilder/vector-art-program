use std::fmt;

use raylib::prelude::*;
use amymath::prelude::*;
use amylib::{prelude::DirectibleDoubleEndedIterator, rc::prelude::*};
use crate::{document::layer::Layer, layer::{BackToFore, LayerType}, shaders::ShaderTable, vector_path::{path_point::{Ctrl1, Ctrl2, PPPart, PathPoint}, VectorPath, DrawPathPoint}, Change, Document};
use super::{HOVER_RADIUS, HOVER_RADIUS_SQR};

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

    pub fn draw(&self, d: &mut impl RaylibDraw, document: &Document, px_world_size: f32, selection_rec: Option<Rectangle>, _shader_table: &ShaderTable) {
        if let Some(selection_rec) = selection_rec {
            // draw selection options
            let selected_idx = self.pt_idx.expect("pt_idx should not be None when selecting with rectangle");
            for (has_selected, path) in document.layers.shallow_iter().cdir::<BackToFore>().filter_map(|layer|
                if let Layer::Path(path) = layer {
                    Some((&self.target == path, path.clone_ref()))
                } else { None })
            {
                let path = path.read();
                path.draw_selected(d, px_world_size);
                for (is_point_selected, pp) in path.curve.points.iter().enumerate().map(|(i, pp)| (has_selected && i == selected_idx, pp)) {
                    let is_selected = is_point_selected || selection_rec.check_collision_point_rec(pp.p);
                    d.draw_path_point(pp, px_world_size, path.settings.color, is_selected, false, false);
                }
            }
        } else {
            let path = self.target.read();
            if let Some(idx) = self.pt_idx {
                path.draw_selected(d, px_world_size);
                for (pp_idx, pp) in path.curve.points.iter().enumerate() {
                    let is_selected = pp_idx == idx;
                    d.draw_path_point(pp, px_world_size, path.settings.color, is_selected,
                        // dont implement this until i can click and drag them directly
                        is_selected/* || pp_idx.checked_sub(1).is_some_and(|prev| prev == idx)*/,
                        is_selected/* || pp_idx.checked_add(1).is_some_and(|next| next == idx)*/,
                    );
                }
            } else {
                path.draw_selected(d, px_world_size * 3.0);
            }
        }
    }
}
