use raylib::prelude::*;
use amylib::rc::*;
use crate::{layer::Layer, vector_path::path_point::{CtrlPt1, CtrlPt2, DistanceSqr, PPPart, PathPoint, ReflectVector}, Change, Document};
use super::HOVER_RADIUS_SQR;

struct EditSinglePointAction {
    target: StrongMut<Layer>,
    idx: usize,
    pre: PathPoint,
    post: PathPoint,
}

impl Change for EditSinglePointAction {
    fn redo(&mut self, _document: &mut Document) -> Result<(), String> {
        let mut target = self.target.write();
        let Layer::Path(path) = &mut *target else { panic!("`target` is required to be a vector path") };
        path.points[self.idx].clone_from(&self.post);
        Ok(())
    }

    fn undo(&mut self, _document: &mut Document) -> Result<(), String> {
        let mut target = self.target.write();
        let Layer::Path(path) = &mut *target else { panic!("`target` is required to be a vector path") };
        path.points[self.idx].clone_from(&self.pre);
        Ok(())
    }
}

pub struct SingleSelect {
    pub target: StrongMut<Layer>,
    pub pt_idx: usize,
    pub part: PPPart,
}

impl SingleSelect {
    pub fn drag(&mut self, delta: Vector2) {
        let mut layer = self.target.write();
        let Layer::Path(path) = &mut *layer else { panic!("point selection must target path") };
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
        let layer = self.target.read();
        let Layer::Path(path) = &*layer else { panic!("point selection must target path") };
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
        (pp.p.distance_sqr_to(mouse_world_pos) <= HOVER_RADIUS_SQR)
            .then_some(PPPart::Anchor)
    }

    pub fn draw(&self, d: &mut impl RaylibDraw, document: &Document, mouse_world_pos: Vector2) {

    }
}
