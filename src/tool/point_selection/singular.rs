use raylib::prelude::*;
use amymath::prelude::*;
use amylib::{prelude::DirectibleDoubleEndedIterator, rc::*};
use crate::{document::layer::LayerData, layer::{BackToFore, LayerType}, vector_path::{path_point::{CtrlPt1, CtrlPt2, PPPart, PathPoint}, VectorPath}, Change, Document};
use super::{multiple::{EnumerateSelectedPoints, SelectionPiece}, HOVER_RADIUS, HOVER_RADIUS_SQR};

struct EditSinglePointAction {
    target: StrongMut<VectorPath>,
    idx: usize,
    pre: PathPoint,
    post: PathPoint,
}

impl Change for EditSinglePointAction {
    fn redo(&mut self, _document: &mut Document) -> Result<(), String> {
        let mut path = self.target.write();
        path.points[self.idx].clone_from(&self.post);
        Ok(())
    }

    fn undo(&mut self, _document: &mut Document) -> Result<(), String> {
        let mut path = self.target.write();
        path.points[self.idx].clone_from(&self.pre);
        Ok(())
    }
}

pub struct SingleSelect {
    pub target: StrongMut<VectorPath>,
    pub pt_idx: usize,
    pub part: PPPart,
}

impl SingleSelect {
    pub fn drag(&mut self, delta: Vector2) {
        let mut path = self.target.write();
        let pp = &mut path.points[self.pt_idx];
        match &self.part {
            PPPart::Anchor => {
                pp.move_point(delta);
            }

            PPPart::Ctrl(side) => {
                let CtrlPt1 { c1: (ref c1_side, c1), c2 } = pp.ctrls.as_mut().expect("should not select corner");
                if c1_side == side {
                    *c1 += delta;
                } else {
                    let c2 = c2.as_mut().expect("should not select corner");
                    match c2 {
                        CtrlPt2::Exact(c2) => *c2 += delta,
                        CtrlPt2::Mirror(ref s2) => *c2 = CtrlPt2::Exact(c1.reflected_to(pp.p, *s2) + delta),
                        CtrlPt2::Smooth => *c2 = CtrlPt2::Exact(c1.reflected_over(pp.p) + delta),
                    }
                }
            }
        }
    }

    pub fn get_selected(&mut self, mouse_world_pos: Vector2) -> Option<PPPart> {
        let path = self.target.read();
        let pp = &path.points[self.pt_idx];
        if let Some(CtrlPt1 { c1: (c1_side, c1), c2 }) = pp.ctrls {
            if c1.distance_sqr_to(mouse_world_pos) <= HOVER_RADIUS_SQR {
                return Some(PPPart::Ctrl(c1_side));
            } else if let Some(c2) = c2 {
                let c2 = match c2 {
                    CtrlPt2::Exact(c2)  => c2,
                    CtrlPt2::Smooth     => c1.reflected_over(pp.p),
                    CtrlPt2::Mirror(s2) => c1.reflected_to(pp.p, s2),
                };
                if c2.distance_sqr_to(mouse_world_pos) <= HOVER_RADIUS_SQR {
                    let c2_side = c1_side.opposite();
                    return Some(PPPart::Ctrl(c2_side));
                }
            }
        }
        (pp.p.rec_distance_to(mouse_world_pos) <= HOVER_RADIUS)
            .then_some(PPPart::Anchor)
    }

    pub fn draw(&self, d: &mut impl RaylibDraw, document: &Document, px_world_size: f32, selection_rec: Option<Rectangle>) {
        if let Some(selection_rec) = selection_rec {
            // draw selection options
            let piece = SelectionPiece { target: self.target.clone_mut(), points: vec![self.pt_idx] };
            for layer in document.layers.shallow_iter().cdir::<BackToFore>() {
                if let LayerData::Path(path) = &layer.data {
                    let selected = (path == &self.target).then(|| &piece);
                    let path = path.read();
                    path.draw_selected(d, px_world_size);
                    if let Some(selected) = selected {
                        for (is_point_selected, pp) in path.enumerate_selected_points(selected) {
                            let is_selected = is_point_selected || selection_rec.check_collision_point_rec(pp.p);
                            pp.draw(d, px_world_size, path.settings.read().color, is_selected, false, false);
                        }
                    } else {
                        for pp in path.points.iter() {
                            let is_selected = selection_rec.check_collision_point_rec(pp.p);
                            pp.draw(d, px_world_size, path.settings.read().color, is_selected, false, false);
                        }
                    }
                }
            }
        } else {
            let path = self.target.read();
            path.draw_selected(d, px_world_size);
            let idx = self.pt_idx;
            for (pp_idx, pp) in path.points.iter().enumerate() {
                let is_selected = pp_idx == idx;
                pp.draw(d, px_world_size, path.settings.read().color, is_selected,
                    // dont implement this until i can click and drag them directly
                    is_selected/* || pp_idx.checked_sub(1).is_some_and(|prev| prev == idx)*/,
                    is_selected/* || pp_idx.checked_add(1).is_some_and(|next| next == idx)*/,
                );
            }
        }
    }
}
