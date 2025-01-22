use raylib::prelude::*;
use crate::{layer::{rc::StrongLayerMut, tree::LayerIterDir, Layer}, vector_path::path_point::{Ctrl, CtrlPt1, CtrlPt2, DistanceSqr, PPPart, PathPoint, ReflectVector}, Document};
use super::ToolType;

pub struct GroupHover {
    pub group_layer: StrongLayerMut,
}

pub enum PathHoverRegion {
    Fill,
    Edge,
    Vert {
        point: usize,
        part: PPPart,
    },
}

pub struct PathHover {
    pub path_layer: StrongLayerMut,
    pub region: PathHoverRegion,
}

pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

pub enum Corner {
    TR,
    BR,
    BL,
    TL,
}

pub enum RasterHoverRegion {
    Object,
    Side {
        side: Side,
    },
    Corner {
        corner: Corner,
    }
}

pub struct RasterHover {
    pub raster_layer: StrongLayerMut,
    pub region: RasterHoverRegion,
}

pub enum Hover {
    Group(GroupHover),
    Path(PathHover),
    Raster(RasterHover),
}

impl Hover {
    pub const fn group(group_layer: StrongLayerMut) -> Self {
        Self::Group(GroupHover { group_layer })
    }

    pub const fn path_fill(path_layer: StrongLayerMut) -> Self {
        Self::Path(PathHover { path_layer, region: PathHoverRegion::Fill })
    }
    pub const fn path_edge(path_layer: StrongLayerMut) -> Self {
        Self::Path(PathHover { path_layer, region: PathHoverRegion::Edge })
    }
    pub const fn path_vert(path_layer: StrongLayerMut, point: usize, part: PPPart) -> Self {
        Self::Path(PathHover { path_layer, region: PathHoverRegion::Vert { point, part } })
    }

    pub const fn raster_object(raster_layer: StrongLayerMut) -> Self {
        Self::Raster(RasterHover { raster_layer, region: RasterHoverRegion::Object })
    }
    pub const fn raster_side(raster_layer: StrongLayerMut, side: Side) -> Self {
        Self::Raster(RasterHover { raster_layer, region: RasterHoverRegion::Side { side } })
    }
    pub const fn raster_corner(raster_layer: StrongLayerMut, corner: Corner) -> Self {
        Self::Raster(RasterHover { raster_layer, region: RasterHoverRegion::Corner { corner } })
    }
}

pub struct HoverOrDrag {
    hover: Hover,
    is_dragging: bool,
}

pub struct DirectSelection {
    /// (path, point, cin vs p vs cout)
    hovered: Option<HoverOrDrag>,
}

impl DirectSelection {
    pub fn new() -> Self {
        Self { hovered: None }
    }
}

pub const HOVER_RADIUS: f32 = 5.0;
pub const HOVER_RADIUS_SQR: f32 = HOVER_RADIUS * HOVER_RADIUS;

pub const SNAP_VERT_RADIUS: f32 = 5.0;
pub const SNAP_VERT_RADIUS_SQR: f32 = SNAP_VERT_RADIUS * SNAP_VERT_RADIUS;

impl ToolType for DirectSelection {
    fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2, _mouse_world_delta: Vector2) {
        let zoom_inv = document.camera.zoom.recip();
        let hover_radius_sqr = HOVER_RADIUS_SQR * zoom_inv * zoom_inv;
        let snap_vert_radius_sqr = SNAP_VERT_RADIUS_SQR * zoom_inv * zoom_inv;
        if let Some(hover) = self.hovered.as_mut() {
            if !hover.is_dragging && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                // - clone
                // - separate path corners
                if rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) {
                    match &mut hover.hover {
                        Hover::Path(PathHover { path_layer, region }) => {
                            let Layer::Path(path) = &mut *path_layer.write() else { panic!("PathHover must reference a Path layer") };
                            match region {
                                PathHoverRegion::Vert { ref point, part: part @ PPPart::Anchor } => {
                                    let pp = &mut path.points[*point];
                                    match pp {
                                        PathPoint { ctrls: Some(CtrlPt1 { c1: (filled_side, _), c2: c2 @ None }), .. } => {
                                            *c2 = Some(CtrlPt2::Exact(mouse_world_pos));
                                            *part = PPPart::Ctrl(filled_side.opposite());
                                        }
                                        PathPoint { ctrls: ctrls @ None, .. } => {
                                            *ctrls = Some(CtrlPt1 { c1: (Ctrl::Out, mouse_world_pos), c2: Some(CtrlPt2::Smooth) });
                                            *part = PPPart::Ctrl(Ctrl::Out);
                                        }
                                        PathPoint { ctrls: Some(CtrlPt1 { c1: _, c2: Some(_) }), .. } => (), // dont do the thing
                                    }
                                }

                                PathHoverRegion::Edge | PathHoverRegion::Fill | PathHoverRegion::Vert { .. } => (),
                            }
                        }

                        Hover::Group(_) => todo!("clone on alt-drag"),

                        Hover::Raster(_) => todo!("clone on alt-drag"),
                    }
                }
                // start dragging
                hover.is_dragging = true;
            } else if hover.is_dragging && rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                // finish dragging
                match &mut hover.hover {
                    Hover::Group(GroupHover { group_layer: _ }) => todo!(),

                    Hover::Path(PathHover { path_layer, ref region }) => {
                        let Layer::Path(path) = &mut *path_layer.write() else { panic!("PathHover must reference a Path layer") };
                        match region {
                            PathHoverRegion::Fill => todo!(),

                            PathHoverRegion::Edge => todo!(),

                            PathHoverRegion::Vert { point, part } => {
                                let pp = &mut path.points[*point];

                                if let PPPart::Ctrl(part) = part {
                                    let CtrlPt1 { c1: (side1, c1), c2 } = pp.ctrls.as_mut().expect("should not drag corner, it should have been made exact when clicked");
                                    if part == side1 {
                                        if c1.distance_sqr_to(pp.p) <= snap_vert_radius_sqr {
                                            // snap to corner
                                            pp.ctrls = if let Some(CtrlPt2::Exact(c2)) = c2.as_ref() {
                                                Some(CtrlPt1 { c1: (side1.opposite(), *c2), c2: None })
                                            } else { None };
                                        } else if let Some(CtrlPt2::Exact(c2)) = c2 {
                                            if c1.distance_sqr_to(c2.reflected_over(pp.p)) <= snap_vert_radius_sqr {
                                                // snap to smooth
                                                pp.ctrls = Some(CtrlPt1 { c1: (side1.opposite(), *c2), c2: Some(CtrlPt2::Smooth) });
                                            } else {
                                                // todo: snap to mirror
                                            }
                                        }
                                    } else {
                                        let Some(CtrlPt2::Exact(c2_pos)) = c2.as_mut() else { panic!("should not drag corner, it should have been made exact when clicked") };
                                        if c2_pos.distance_sqr_to(pp.p) <= snap_vert_radius_sqr {
                                            // snap to corner
                                            *c2 = None;
                                        } else if c2_pos.distance_sqr_to(c1.reflected_over(pp.p)) <= snap_vert_radius_sqr {
                                            // snap to smooth
                                            *c2 = Some(CtrlPt2::Smooth);
                                        } else {
                                            // todo: snap to mirror
                                        }
                                    };
                                }
                            }
                        }
                    }

                    Hover::Raster(RasterHover { raster_layer: _, region: ref _region }) => todo!(),
                }
                hover.is_dragging = false;
            }
        }

        match self.hovered.as_mut() {
            Some(HoverOrDrag { hover, is_dragging: true }) => {
                match hover {
                    Hover::Group(GroupHover { group_layer }) => {
                        let Layer::Group(_group) = &mut *group_layer.write() else { panic!("GroupHover must reference a Group layer") };
                        todo!()
                    }
                    Hover::Path(PathHover { path_layer, ref region }) => {
                        let Layer::Path(path) = &mut *path_layer.write() else { panic!("PathHover must reference a Path layer") };
                        match region {
                            PathHoverRegion::Fill => {
                                todo!()
                            }
                            PathHoverRegion::Edge => {
                                todo!()
                            }
                            PathHoverRegion::Vert { point, part } => {
                                let pp = &mut path.points[*point];
                                match part {
                                    PPPart::Anchor => pp.set_point(mouse_world_pos),
                                    PPPart::Ctrl(side) => {
                                        let CtrlPt1 { c1, c2 } = pp.ctrls.as_mut().expect("should not hover ctrl if there is none");
                                        if c1.0 == *side {
                                            c1.1 = mouse_world_pos;
                                        } else {
                                            *c2 = Some(CtrlPt2::Exact(mouse_world_pos));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Hover::Raster(RasterHover { raster_layer, region: ref _region }) => {
                        let Layer::Raster(_raster) = &mut *raster_layer.write() else { panic!("RasterHover must reference a Raster layer") };
                        todo!()
                    }
                }
            }

            Some(HoverOrDrag { is_dragging: false, .. }) | None => {
                self.hovered = document.layers
                    .tree_iter_mut(
                        LayerIterDir::ForeToBack,
                        |group| !group.settings.is_hidden && !group.settings.is_locked)
                    .find_map(|(layer_rc, _depth)| -> Option<Hover> {
                        match &*layer_rc.read() {
                            Layer::Group(_group) => {
                                // todo
                                None
                            },
                            Layer::Path(path) => {
                                path.points.iter()
                                    .enumerate()
                                    .find_map(|(i, pp)| {
                                        // if c_in or c_out is a corner, then p will match first
                                        let (c_in, p, c_out) = pp.calculate();
                                        [(p, PPPart::Anchor), (c_in, PPPart::Ctrl(Ctrl::In)), (c_out, PPPart::Ctrl(Ctrl::Out))]
                                            .into_iter()
                                            .find_map(|(p, sect)| (p.distance_sqr_to(mouse_world_pos) <= hover_radius_sqr)
                                                .then(|| Hover::path_vert(layer_rc.clone_mut(), i, sect)))
                                    })
                                    .or_else(|| {
                                        // todo: fill/edge
                                        None
                                    })
                            }
                            Layer::Raster(_raster) => {
                                // todo
                                None
                            }
                        }
                    })
                    .map(|hover| HoverOrDrag { hover, is_dragging: false });
            }
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, document: &Document, _mouse_world_pos: Vector2) {
        let zoom_inv = document.camera.zoom.recip();
        let snap_vert_radius_sqr = SNAP_VERT_RADIUS_SQR * zoom_inv * zoom_inv;
        if let Some(HoverOrDrag { hover, is_dragging }) = &self.hovered {
            match hover {
                Hover::Group(GroupHover { group_layer }) => {
                    let Layer::Group(_group) = &*group_layer.read() else { panic!("GroupHover must reference a Group layer") };
                    todo!()
                }
                Hover::Path(PathHover { path_layer, region }) => {
                    let Layer::Path(path) = &*path_layer.read() else { panic!("PathHover must reference a Path layer") };
                    match region {
                        PathHoverRegion::Fill => todo!(),

                        PathHoverRegion::Edge => todo!(),

                        PathHoverRegion::Vert { point, part } => {
                            let pp = &path.points[*point];
                            let p = pp.p;
                            match part {
                                PPPart::Anchor => {
                                    d.draw_circle_v(p, HOVER_RADIUS * zoom_inv, Color::DODGERBLUE);
                                }
                                PPPart::Ctrl(part) => {
                                    let mut draw_ctrl_exact = |mut c_self: Vector2, c_opp: Option<&CtrlPt2>| {
                                        // preview snapping to smooth/corner
                                        if let Some(CtrlPt2::Exact(c_opp)) = c_opp {
                                            let c_self_smooth = c_opp.reflected_over(p);
                                            let mirror_dir = (p - *c_opp).normalized();
                                            let c_self_mirror = p + mirror_dir * (c_self - p).dot(mirror_dir);
                                            d.draw_line_v(p, c_self_smooth, Color::DODGERBLUE.alpha(0.5));
                                            d.draw_ring(c_self_smooth, (SNAP_VERT_RADIUS - 2.0) * zoom_inv, SNAP_VERT_RADIUS * zoom_inv, 0.0, 360.0, 10, Color::DODGERBLUE.alpha(0.5));
                                            d.draw_ring(c_self_mirror, (SNAP_VERT_RADIUS - 1.0) * zoom_inv, SNAP_VERT_RADIUS * zoom_inv, 0.0, 360.0, 10, Color::BLUEVIOLET.alpha(0.5));
                                            if c_self.distance_sqr_to(c_self_smooth) <= snap_vert_radius_sqr {
                                                c_self = c_self_smooth;
                                            } else if c_self.distance_sqr_to(c_self_mirror) <= snap_vert_radius_sqr {
                                                c_self = c_self_mirror;
                                            }
                                        }
                                        d.draw_circle_v(c_self, if !is_dragging { HOVER_RADIUS } else { SNAP_VERT_RADIUS } * zoom_inv, Color::DODGERBLUE);
                                    };

                                    let CtrlPt1 { c1: (side1, c1), c2 } = pp.ctrls.as_ref().expect("should not hover ctrl of corner");
                                    if part == side1 {
                                        draw_ctrl_exact(*c1, c2.as_ref());
                                    } else {
                                        let c2 = c2.as_ref().expect("should not hover ctrl of corner");
                                        match c2 {
                                            CtrlPt2::Smooth => {
                                                let c_self_smooth = c1.reflected_over(p);
                                                d.draw_circle_v(c_self_smooth, if !is_dragging { HOVER_RADIUS } else { SNAP_VERT_RADIUS } * zoom_inv, Color::DODGERBLUE);
                                            }

                                            CtrlPt2::Mirror(s2) => {
                                                let c_self_smooth = c1.reflected_to(p, *s2);
                                                d.draw_circle_v(c_self_smooth, if !is_dragging { HOVER_RADIUS } else { SNAP_VERT_RADIUS } * zoom_inv, Color::DODGERBLUE);
                                            }

                                            CtrlPt2::Exact(c2) => {
                                                draw_ctrl_exact(*c2, Some(CtrlPt2::Exact(*c1)).as_ref());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Hover::Raster(RasterHover { raster_layer, region }) => {
                    let Layer::Raster(_raster) = &*raster_layer.read() else { panic!("RasterHover must reference a Raster layer") };
                    match region {
                        RasterHoverRegion::Object => todo!(),
                        RasterHoverRegion::Side { side } => match side {
                            Side::Top    => todo!(),
                            Side::Right  => todo!(),
                            Side::Bottom => todo!(),
                            Side::Left   => todo!(),
                        }
                        RasterHoverRegion::Corner { corner } => match corner {
                            Corner::TR => todo!(),
                            Corner::BR => todo!(),
                            Corner::BL => todo!(),
                            Corner::TL => todo!(),
                        }
                    }
                }
            }
        }
    }
}
