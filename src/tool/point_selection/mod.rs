use amymath::prelude::*;
use raylib::prelude::*;
use amylib::iter::directed::DirectibleDoubleEndedIterator;
use crate::{document::layer::Layer, layer::{BackToFore, ForeToBack, LayerType}, shaders::ShaderTable, vector_path::{path_point::PPPart, DrawPathPoint}, Document};
use super::ToolType;

pub const HOVER_RADIUS: f32 = 3.0;
pub const HOVER_RADIUS_SQR: f32 = HOVER_RADIUS * HOVER_RADIUS;

// pub const SNAP_VERT_RADIUS: f32 = 3.0;
// pub const SNAP_VERT_RADIUS_SQR: f32 = SNAP_VERT_RADIUS * SNAP_VERT_RADIUS;

mod singular;
mod multiple;

use singular::SingleSelect;
use multiple::{MultiSelect, SelectionPiece};

/// Pieces should be ordered by unique target layer [`TreeIterDir::BackToFore`]. Points should be ordered by index.
enum Selection {
    Singular(SingleSelect),
    Multiple(MultiSelect),
}

struct SelectionState {
    pub selection: Selection,
    pub drag: Option<(Vector2, Vector2)>, // (start, cumulative)
}

impl SelectionState {
    pub fn drag(&mut self, delta: Vector2) {
        match &mut self.selection {
            Selection::Singular(x) => x.drag(delta),
            Selection::Multiple(x) => x.drag(delta),
        }
    }
}

pub struct PointSelection {
    state: Option<SelectionState>,
    selection_points: Option<(Vector2, Vector2)>,
}

impl PointSelection {
    pub const fn new() -> Self {
        Self {
            state: None,
            selection_points: None,
        }
    }

    fn begin_dragging(&mut self, document: &mut Document, mouse_world_pos: Vector2) {
        let drag = Some((mouse_world_pos, mouse_world_pos));

        if let Some(state) = self.state.as_mut() {
            match &mut state.selection {
                Selection::Singular(x) => {
                    if let Some(part) = x.get_selected(mouse_world_pos) {
                        x.part = part;
                        state.drag = drag;
                        return;
                    }
                },

                Selection::Multiple(x) => {
                    if x.is_selected(mouse_world_pos) {
                        state.drag = drag;
                        return;
                    }
                }
            }
        }

        // // test
        // let mouse_world_pos = Vector2::from(unsafe { ffi::GetScreenToWorld2D(ffi::GetMousePosition(), document.camera.into()) });
        // d.draw_circle_v(mouse_world_pos, 5.0, Color::RED);
        // for layer in document.layers.shallow_iter().cdir::<BackToFore>() {
        //     if let Layer::Path(path) = layer {
        //         let path = path.read();
        //         for bez in path.curve.slices() {
        //             if bez.bounds().grow(HOVER_RADIUS_SQR).is_overlapping_point(mouse_world_pos) &&
        //                 let Some((_t, p)) = bez.estimate_time_at(mouse_world_pos) &&
        //                 p.distance_sqr_to(mouse_world_pos) <= HOVER_RADIUS_SQR
        //             {
        //                 d.draw_circle_v(p, 4.0, Color::GREEN);
        //             }
        //         }
        //     }
        // }

        let hovered_point = document.layers
            .shallow_iter_mut()
            .cdir::<ForeToBack>()
            .find_map(|layer| {
                if let Layer::Path(path) = layer {
                    let path_borrow = path.read();
                    let curve = &path_borrow.curve;
                    let idx = curve.points.iter()
                        .position(|pp| pp.p.rec_distance_to(mouse_world_pos) <= HOVER_RADIUS);
                    if idx.is_some() || curve.slices()
                        .any(|bez|
                            bez.bounds().grow(HOVER_RADIUS).is_overlapping_point(mouse_world_pos) &&
                            bez.estimate_time_at(mouse_world_pos).is_some_and(|(_, p)| p.distance_sqr_to(mouse_world_pos) <= HOVER_RADIUS_SQR))
                    {
                        drop(path_borrow);
                        return Some((path, idx));
                    }
                }
                None
            });

        if let Some((hovered_target, hovered_idx)) = hovered_point {
            self.state = Some(SelectionState {
                selection: Selection::Singular(SingleSelect {
                    target: hovered_target.clone_mut(),
                    pt_idx: hovered_idx,
                    part: PPPart::Anchor,
                }),
                drag,
            });
        } else {
            self.state = None;
            self.selection_points = Some((mouse_world_pos, mouse_world_pos));
        }
    }

    fn end_dragging(&mut self, document: &mut Document, mouse_world_pos: Vector2) {
        if let Some(state) = self.state.as_mut() {
            state.drag = None;
        }

        // finalize selection
        if let Some((selection_start, _)) = self.selection_points.take() {
            let selection_rec = selection_start.minmax_rec(mouse_world_pos);

            let selected = document.layers
                .shallow_iter_mut()
                .filter_map(|layer| {
                    if let Layer::Path(path) = layer {
                        let points = path.read().curve.points.iter()
                            .enumerate()
                            .filter_map(|(idx, pp)|
                                selection_rec.check_collision_point_rec(pp.p)
                                    .then_some(idx))
                            .collect::<Vec<usize>>();

                        if !points.is_empty() {
                            return Some(SelectionPiece { target: path.clone_mut(), points, });
                        }
                    }
                    None
                })
                .collect::<Vec<SelectionPiece>>();

            self.state = (!selected.is_empty())
                .then(|| {
                    SelectionState {
                        selection: match &selected[..] {
                            [SelectionPiece { target, points }] if points.len() == 1
                                => Selection::Singular(SingleSelect { target: target.clone_mut(), pt_idx: Some(points[0]), part: PPPart::Anchor }),
                            [..]
                                => Selection::Multiple(MultiSelect { pieces: selected }),
                        },
                        drag: None,
                    }
                });
        }
    }
}

impl ToolType for PointSelection {
    fn tick(&mut self, rl: &mut RaylibHandle, _thread: &RaylibThread, document: &mut Document, mouse_world_pos: Vector2) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            self.begin_dragging(document, mouse_world_pos);
        }

        if let Some((_, curr)) = self.selection_points.as_mut() {
            *curr = mouse_world_pos;
        }

        if let Some(state) = self.state.as_mut() && let Some((_drag_start, drag_cum)) = state.drag.as_mut() {
            let delta = mouse_world_pos - *drag_cum;
            *drag_cum += delta;
            state.drag(delta);
        }

        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            self.end_dragging(document, mouse_world_pos);
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, document: &Document, shader_table: &ShaderTable) {
        let px_world_size = document.camera.zoom.recip();

        let selection_rec = self.selection_points.as_ref().copied().map(|(start, end)|
            start.minmax_rec(end)
        );

        if let Some(selection_rec) = selection_rec.as_ref() {
            d.draw_rectangle_rec(selection_rec, Color::BLUE.alpha(0.125));
        }

        // goal: draw each control point ONLY ONCE without O(n^2) complexity selection test
        // IMPORTANT: relies on previously stated sorting requirements

        match self.state.as_ref() {
            Some(SelectionState { selection: Selection::Singular(selected), .. }) => {
                selected.draw(d, document, px_world_size, selection_rec, shader_table);
            }

            Some(SelectionState { selection: Selection::Multiple(selected), .. }) => {
                selected.draw(d, document, px_world_size, selection_rec, shader_table);
            }

            None => {
                // draw selection options
                for layer in document.layers.shallow_iter().cdir::<BackToFore>() {
                    if let Layer::Path(path) = layer {
                        let path = path.read();
                        path.draw_selected(d, px_world_size);
                        for pp in &path.curve.points {
                            let is_selected = selection_rec.is_some_and(|rec| rec.check_collision_point_rec(pp.p));
                            d.draw_path_point(pp, px_world_size, path.settings.color, is_selected, false, false);
                        }
                    }
                }
            }
        }
    }
}




// pub struct GroupHover {
//     pub group_layer: StrongMut<Layer>,
// }

// pub enum PathHoverRegion {
//     Fill,
//     Edge,
//     Vert {
//         point: usize,
//         part: PPPart,
//     },
// }

// pub struct PathHover {
//     pub path_layer: StrongMut<Layer>,
//     pub region: PathHoverRegion,
// }

// pub enum Side {
//     Top,
//     Right,
//     Bottom,
//     Left,
// }

// pub enum Corner {
//     TR,
//     BR,
//     BL,
//     TL,
// }

// pub enum RasterHoverRegion {
//     Object,
//     Side {
//         side: Side,
//     },
//     Corner {
//         corner: Corner,
//     }
// }

// pub struct RasterHover {
//     pub raster_layer: StrongMut<Layer>,
//     pub region: RasterHoverRegion,
// }

// pub enum Hover {
//     Group(GroupHover),
//     Path(PathHover),
//     Raster(RasterHover),
// }

// impl Hover {
//     pub const fn group(group_layer: StrongMut<Layer>) -> Self {
//         Self::Group(GroupHover { group_layer })
//     }

//     pub const fn path_fill(path_layer: StrongMut<Layer>) -> Self {
//         Self::Path(PathHover { path_layer, region: PathHoverRegion::Fill })
//     }
//     pub const fn path_edge(path_layer: StrongMut<Layer>) -> Self {
//         Self::Path(PathHover { path_layer, region: PathHoverRegion::Edge })
//     }
//     pub const fn path_vert(path_layer: StrongMut<Layer>, point: usize, part: PPPart) -> Self {
//         Self::Path(PathHover { path_layer, region: PathHoverRegion::Vert { point, part } })
//     }

//     pub const fn raster_object(raster_layer: StrongMut<Layer>) -> Self {
//         Self::Raster(RasterHover { raster_layer, region: RasterHoverRegion::Object })
//     }
//     pub const fn raster_side(raster_layer: StrongMut<Layer>, side: Side) -> Self {
//         Self::Raster(RasterHover { raster_layer, region: RasterHoverRegion::Side { side } })
//     }
//     pub const fn raster_corner(raster_layer: StrongMut<Layer>, corner: Corner) -> Self {
//         Self::Raster(RasterHover { raster_layer, region: RasterHoverRegion::Corner { corner } })
//     }
// }

// pub struct HoverOrDrag {
//     hover: Hover,
//     is_dragging: bool,
// }

// pub struct DirectSelection {
//     hovered: Option<HoverOrDrag>,
// }

// impl DirectSelection {
//     pub fn new() -> Self {
//         Self { hovered: None }
//     }

//     fn clone_and_separate_path_corners(hover: &mut HoverOrDrag, mouse_world_pos: Vector2) {
//         match &mut hover.hover {
//             Hover::Path(PathHover { path_layer, region }) => {
//                 let Layer::Path(path) = &mut *path_layer.write() else { panic!("PathHover must reference a Path layer") };
//                 match region {
//                     PathHoverRegion::Vert { ref point, part: part @ PPPart::Anchor } => {
//                         let pp = &mut path.points[*point];
//                         match pp {
//                             PathPoint { ctrls: Some(CtrlPt1 { c1: (filled_side, _), c2: c2 @ None }), .. } => {
//                                 *c2 = Some(CtrlPt2::Exact(mouse_world_pos));
//                                 *part = PPPart::Ctrl(filled_side.opposite());
//                             }

//                             PathPoint { ctrls: ctrls @ None, .. } => {
//                                 *ctrls = Some(CtrlPt1 { c1: (Ctrl::Out, mouse_world_pos), c2: Some(CtrlPt2::Reflect) });
//                                 *part = PPPart::Ctrl(Ctrl::Out);
//                             }

//                             PathPoint { ctrls: Some(CtrlPt1 { c1: _, c2: Some(_) }), .. } => (), // dont do the thing
//                         }
//                     }

//                     PathHoverRegion::Edge | PathHoverRegion::Fill | PathHoverRegion::Vert { .. } => (),
//                 }
//             }

//             Hover::Group(_) => todo!("clone on alt-drag"),

//             Hover::Raster(_) => todo!("clone on alt-drag"),
//         }
//     }

//     fn begin_dragging(rl: &mut RaylibHandle, hover: &mut HoverOrDrag, mouse_world_pos: Vector2) {
//         if rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) {
//             Self::clone_and_separate_path_corners(hover, mouse_world_pos);
//         }
//         hover.is_dragging = true;
//     }

//     fn end_dragging_vert(path: &mut VectorPath, point: &usize, part: &PPPart, snap_vert_radius_sqr: f32) {
//         let pp = &mut path.points[*point];

//         if let PPPart::Ctrl(part) = part {
//             fn snap_to_reflect_or_mirror(pp: &mut PathPoint, side_self: Ctrl, c_self: Vector2, c_opp: Vector2, snap_vert_radius_sqr: f32) {
//                 if c_self.distance_sqr_to(c_opp.reflected_over(pp.p)) <= snap_vert_radius_sqr {
//                     // snap to reflect
//                     pp.ctrls = Some(CtrlPt1 { c1: (side_self.opposite(), c_opp), c2: Some(CtrlPt2::Reflect) });
//                 } else {
//                     // snap to mirror
//                     let mirror_dir = (pp.p - c_opp).normalized();
//                     let s_self = (c_self - pp.p).dot(mirror_dir);
//                     let c_self_mirror = pp.p + mirror_dir * s_self;
//                     if c_self.distance_sqr_to(c_self_mirror) <= snap_vert_radius_sqr {
//                         pp.ctrls = Some(CtrlPt1 { c1: (side_self.opposite(), c_opp), c2: Some(CtrlPt2::Mirror(s_self)) });
//                     }
//                 }
//             }

//             let CtrlPt1 { c1: (side1, c1), c2 } = pp.ctrls.as_mut().expect("should not drag corner, it should have been made exact when clicked");
//             if part == side1 {
//                 if c1.distance_sqr_to(pp.p) <= snap_vert_radius_sqr {
//                     // snap to corner
//                     pp.ctrls = if let Some(CtrlPt2::Exact(c2)) = c2.as_ref() {
//                         Some(CtrlPt1 { c1: (side1.opposite(), *c2), c2: None })
//                     } else { None };
//                 } else if let Some(CtrlPt2::Exact(c2)) = c2 {
//                     let (c_self, c_opp) = (*c1, *c2);
//                     snap_to_reflect_or_mirror(pp, *part, c_self, c_opp, snap_vert_radius_sqr);
//                 }
//             } else {
//                 let Some(CtrlPt2::Exact(c2)) = c2.as_ref() else { panic!("should not drag corner, it should have been made exact when clicked") };
//                 let (c_self, c_opp) = (*c2, *c1);
//                 snap_to_reflect_or_mirror(pp, *part, c_self, c_opp, snap_vert_radius_sqr);
//             };
//         }
//     }

//     fn end_dragging(hover: &mut HoverOrDrag, snap_vert_radius_sqr: f32) {
//         match &mut hover.hover {
//             Hover::Group(GroupHover { group_layer: _ }) => todo!(),

//             Hover::Path(PathHover { path_layer, ref region }) => {
//                 let Layer::Path(path) = &mut *path_layer.write() else { panic!("PathHover must reference a Path layer") };
//                 match region {
//                     PathHoverRegion::Fill => todo!(),
//                     PathHoverRegion::Edge => todo!(),
//                     PathHoverRegion::Vert { point, part } => Self::end_dragging_vert(path, point, part, snap_vert_radius_sqr),
//                 }
//             }

//             Hover::Raster(RasterHover { raster_layer: _, region: ref _region }) => todo!(),
//         }
//         hover.is_dragging = false;
//     }

//     fn tick_dragging(hover: &mut Hover, mouse_world_pos: Vector2) {
//         match hover {
//             Hover::Group(GroupHover { group_layer }) => {
//                 let Layer::Group(_group) = &mut *group_layer.write() else { panic!("GroupHover must reference a Group layer") };
//                 todo!()
//             }
//             Hover::Path(PathHover { path_layer, ref region }) => {
//                 let Layer::Path(path) = &mut *path_layer.write() else { panic!("PathHover must reference a Path layer") };
//                 match region {
//                     PathHoverRegion::Fill => {
//                         todo!()
//                     }
//                     PathHoverRegion::Edge => {
//                         todo!()
//                     }
//                     PathHoverRegion::Vert { point, part } => {
//                         let pp = &mut path.points[*point];
//                         match part {
//                             PPPart::Anchor => pp.set_point(mouse_world_pos),
//                             PPPart::Ctrl(side) => {
//                                 let CtrlPt1 { c1, c2 } = pp.ctrls.as_mut().expect("should not hover ctrl if there is none");
//                                 if c1.0 == *side {
//                                     c1.1 = mouse_world_pos;
//                                 } else {
//                                     *c2 = Some(CtrlPt2::Exact(mouse_world_pos));
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             Hover::Raster(RasterHover { raster_layer, region: ref _region }) => {
//                 let Layer::Raster(_raster) = &mut *raster_layer.write() else { panic!("RasterHover must reference a Raster layer") };
//                 todo!()
//             }
//         }
//     }

//     fn tick_hovering(&mut self, document: &mut Document, mouse_world_pos: Vector2, hover_radius_sqr: f32) {
//         self.hovered = document.layers
//             .tree_iter_mut(
//                 TreeIterDir::ForeToBack,
//                 |group| !group.settings.is_hidden && !group.settings.is_locked)
//             .find_map(|(layer_rc, _depth)| -> Option<Hover> {
//                 match &*layer_rc.read() {
//                     Layer::Group(_group) => {
//                         // todo
//                         None
//                     },
//                     Layer::Path(path) => {
//                         path.points.iter()
//                             .enumerate()
//                             .find_map(|(i, pp)| {
//                                 // if c_in or c_out is a corner, then p will match first
//                                 let (c_in, p, c_out) = pp.calculate();
//                                 [(p, PPPart::Anchor), (c_in, PPPart::Ctrl(Ctrl::In)), (c_out, PPPart::Ctrl(Ctrl::Out))]
//                                     .into_iter()
//                                     .find_map(|(p, sect)| (p.distance_sqr_to(mouse_world_pos) <= hover_radius_sqr)
//                                         .then(|| Hover::path_vert(layer_rc.clone_mut(), i, sect)))
//                             })
//                             .or_else(|| {
//                                 // todo: fill/edge
//                                 None
//                             })
//                     }
//                     Layer::Raster(_raster) => {
//                         // todo
//                         None
//                     }
//                 }
//             })
//             .map(|hover| HoverOrDrag { hover, is_dragging: false });
//     }
// }

// pub const HOVER_RADIUS: f32 = 5.0;
// pub const HOVER_RADIUS_SQR: f32 = HOVER_RADIUS * HOVER_RADIUS;

// pub const SNAP_VERT_RADIUS: f32 = 5.0;
// pub const SNAP_VERT_RADIUS_SQR: f32 = SNAP_VERT_RADIUS * SNAP_VERT_RADIUS;

// impl ToolType for DirectSelection {
//     fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2, _mouse_world_delta: Vector2) {
//         let px_world_size = document.camera.zoom.recip();
//         let hover_radius_sqr = HOVER_RADIUS_SQR * px_world_size * px_world_size;
//         let snap_vert_radius_sqr = SNAP_VERT_RADIUS_SQR * px_world_size * px_world_size;
//         if let Some(hover) = self.hovered.as_mut() {
//             if !hover.is_dragging && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
//                 Self::begin_dragging(rl, hover, mouse_world_pos);
//             } else if hover.is_dragging && rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
//                 Self::end_dragging(hover, snap_vert_radius_sqr);
//             }
//         }

//         match self.hovered.as_mut() {
//             Some(HoverOrDrag { hover, is_dragging: true }) => Self::tick_dragging(hover, mouse_world_pos),
//             None | Some(HoverOrDrag { is_dragging: false, .. }) => self.tick_hovering(document, mouse_world_pos, hover_radius_sqr),
//         }
//     }

//     fn draw(&self, d: &mut impl RaylibDraw, document: &Document, _mouse_world_pos: Vector2) {
//         let px_world_size = document.camera.zoom.recip();
//         let snap_vert_radius_sqr = SNAP_VERT_RADIUS_SQR * px_world_size * px_world_size;
//         if let Some(HoverOrDrag { hover, is_dragging }) = &self.hovered {
//             match hover {
//                 Hover::Group(GroupHover { group_layer }) => {
//                     let Layer::Group(_group) = &*group_layer.read() else { panic!("GroupHover must reference a Group layer") };
//                     todo!()
//                 }
//                 Hover::Path(PathHover { path_layer, region }) => {
//                     let Layer::Path(path) = &*path_layer.read() else { panic!("PathHover must reference a Path layer") };
//                     match region {
//                         PathHoverRegion::Fill => todo!(),

//                         PathHoverRegion::Edge => todo!(),

//                         PathHoverRegion::Vert { point, part } => {
//                             let pp = &path.points[*point];
//                             let p = pp.p;
//                             match part {
//                                 PPPart::Anchor => {
//                                     d.draw_circle_v(p, HOVER_RADIUS * px_world_size, Color::DODGERBLUE);
//                                 }
//                                 PPPart::Ctrl(part) => {
//                                     let mut draw_ctrl_exact = |mut c_self: Vector2, c_opp: Option<&CtrlPt2>| {
//                                         // preview snapping to reflect/corner
//                                         if let Some(CtrlPt2::Exact(c_opp)) = c_opp {
//                                             let c_self_reflect = c_opp.reflected_over(p);
//                                             let mirror_dir = (p - *c_opp).normalized();
//                                             let c_self_mirror = p + mirror_dir * (c_self - p).dot(mirror_dir);
//                                             d.draw_line_v(p, c_self_reflect, Color::DODGERBLUE.alpha(0.5));
//                                             d.draw_ring(c_self_reflect, (SNAP_VERT_RADIUS - 2.0) * px_world_size, SNAP_VERT_RADIUS * px_world_size, 0.0, 360.0, 10, Color::DODGERBLUE.alpha(0.5));
//                                             d.draw_ring(c_self_mirror, (SNAP_VERT_RADIUS - 1.0) * px_world_size, SNAP_VERT_RADIUS * px_world_size, 0.0, 360.0, 10, Color::BLUEVIOLET.alpha(0.5));
//                                             if c_self.distance_sqr_to(c_self_reflect) <= snap_vert_radius_sqr {
//                                                 c_self = c_self_reflect;
//                                             } else if c_self.distance_sqr_to(c_self_mirror) <= snap_vert_radius_sqr {
//                                                 c_self = c_self_mirror;
//                                             }
//                                         }
//                                         d.draw_circle_v(c_self, if !is_dragging { HOVER_RADIUS } else { SNAP_VERT_RADIUS } * px_world_size, Color::DODGERBLUE);
//                                     };

//                                     let CtrlPt1 { c1: (side1, c1), c2 } = pp.ctrls.as_ref().expect("should not hover ctrl of corner");
//                                     if part == side1 {
//                                         draw_ctrl_exact(*c1, c2.as_ref());
//                                     } else {
//                                         let c2 = c2.as_ref().expect("should not hover ctrl of corner");
//                                         match c2 {
//                                             CtrlPt2::Reflect => {
//                                                 let c_self_reflect = c1.reflected_over(p);
//                                                 d.draw_circle_v(c_self_reflect, if !is_dragging { HOVER_RADIUS } else { SNAP_VERT_RADIUS } * px_world_size, Color::DODGERBLUE);
//                                             }

//                                             CtrlPt2::Mirror(s2) => {
//                                                 let c_self_reflect = c1.reflected_to(p, *s2);
//                                                 d.draw_circle_v(c_self_reflect, if !is_dragging { HOVER_RADIUS } else { SNAP_VERT_RADIUS } * px_world_size, Color::DODGERBLUE);
//                                             }

//                                             CtrlPt2::Exact(c2) => {
//                                                 draw_ctrl_exact(*c2, Some(CtrlPt2::Exact(*c1)).as_ref());
//                                             }
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//                 Hover::Raster(RasterHover { raster_layer, region }) => {
//                     let Layer::Raster(_raster) = &*raster_layer.read() else { panic!("RasterHover must reference a Raster layer") };
//                     match region {
//                         RasterHoverRegion::Object => todo!(),
//                         RasterHoverRegion::Side { side } => match side {
//                             Side::Top    => todo!(),
//                             Side::Right  => todo!(),
//                             Side::Bottom => todo!(),
//                             Side::Left   => todo!(),
//                         }
//                         RasterHoverRegion::Corner { corner } => match corner {
//                             Corner::TR => todo!(),
//                             Corner::BR => todo!(),
//                             Corner::BL => todo!(),
//                             Corner::TL => todo!(),
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
