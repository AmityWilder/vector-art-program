use raylib::prelude::*;
use crate::{layer::{tree::LayerIterDir, Layer, StrongLayer}, vector_path::path_point::{CtrlPoint, DistanceSqr, ReflectVector}, Document};
use super::ToolType;

pub struct GroupHover {
    pub group_layer: StrongLayer,
}

pub enum PointPart {
    CtrlIn,
    Point,
    CtrlOut,
}

pub enum PathHoverRegion {
    Fill,
    Edge,
    Vert {
        point: usize,
        part: PointPart,
    },
}

pub struct PathHover {
    pub path_layer: StrongLayer,
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
    pub raster_layer: StrongLayer,
    pub region: RasterHoverRegion,
}

pub enum Hover {
    Group(GroupHover),
    Path(PathHover),
    Raster(RasterHover),
}

impl Hover {
    pub const fn group(group_layer: StrongLayer) -> Self {
        Self::Group(GroupHover { group_layer })
    }

    pub const fn path_fill(path_layer: StrongLayer) -> Self {
        Self::Path(PathHover { path_layer, region: PathHoverRegion::Fill })
    }
    pub const fn path_edge(path_layer: StrongLayer) -> Self {
        Self::Path(PathHover { path_layer, region: PathHoverRegion::Edge })
    }
    pub const fn path_vert(path_layer: StrongLayer, point: usize, part: PointPart) -> Self {
        Self::Path(PathHover { path_layer, region: PathHoverRegion::Vert { point, part } })
    }

    pub const fn raster_object(raster_layer: StrongLayer) -> Self {
        Self::Raster(RasterHover { raster_layer, region: RasterHoverRegion::Object })
    }
    pub const fn raster_side(raster_layer: StrongLayer, side: Side) -> Self {
        Self::Raster(RasterHover { raster_layer, region: RasterHoverRegion::Side { side } })
    }
    pub const fn raster_corner(raster_layer: StrongLayer, corner: Corner) -> Self {
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

pub const SNAP_VERT_RADIUS: f32 = HOVER_RADIUS * 0.5;
pub const SNAP_VERT_RADIUS_SQR: f32 = SNAP_VERT_RADIUS * SNAP_VERT_RADIUS;

impl ToolType for DirectSelection {
    fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2) {
        if let Some(hover) = self.hovered.as_mut() {
            if !hover.is_dragging && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                // - clone
                // - separate path corners
                if rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) {
                    match &mut hover.hover {
                        Hover::Path(PathHover { path_layer, region }) => {
                            let Layer::Path(path) = &mut *path_layer.borrow_mut() else { panic!("PathHover must reference a Path layer") };
                            match region {
                                PathHoverRegion::Vert { ref point, part: part @ PointPart::Point } => {
                                    let pp = &mut path.points[*point];
                                    match (&pp.c_in, &pp.c_out) {
                                        (CtrlPoint::Exact(_), CtrlPoint::Corner) => {
                                            pp.c_out = CtrlPoint::Exact(mouse_world_pos);
                                            *part = PointPart::CtrlOut;
                                        }
                                        (CtrlPoint::Corner, CtrlPoint::Exact(_)) => {
                                            pp.c_in = CtrlPoint::Exact(mouse_world_pos);
                                            *part = PointPart::CtrlIn;
                                        }
                                        (CtrlPoint::Corner, CtrlPoint::Corner) => {
                                            pp.c_in = CtrlPoint::Smooth;
                                            pp.c_out = CtrlPoint::Exact(mouse_world_pos);
                                            *part = PointPart::CtrlOut;
                                        }

                                        _ => (),
                                    }
                                }

                                _ => (),
                            }
                        }

                        _ => (),
                    }
                }
                // start dragging
                hover.is_dragging = true;
            } else if hover.is_dragging && rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                // finish dragging
                match &mut hover.hover {
                    Hover::Group(GroupHover { group_layer: _ }) => todo!(),

                    Hover::Path(PathHover { path_layer, ref region }) => {
                        let Layer::Path(path) = &mut *path_layer.borrow_mut() else { panic!("PathHover must reference a Path layer") };
                        match region {
                            PathHoverRegion::Fill => todo!(),

                            PathHoverRegion::Edge => todo!(),

                            PathHoverRegion::Vert { point, part } => {
                                let pp = &mut path.points[*point];
                                match part {
                                    PointPart::Point => (), // no cleanup needed

                                    PointPart::CtrlIn | PointPart::CtrlOut => {
                                        let (c_self, c_opp) = match part {
                                            PointPart::CtrlIn => (&mut pp.c_in, &mut pp.c_out),
                                            PointPart::CtrlOut => (&mut pp.c_out, &mut pp.c_in),
                                            PointPart::Point => unreachable!(),
                                        };
                                        if let CtrlPoint::Exact(c_self_ex) = &c_self {
                                            if c_self_ex.distance_sqr_to(pp.p) <= SNAP_VERT_RADIUS_SQR {
                                                // snap to corner
                                                *c_self = CtrlPoint::Corner;
                                                match &c_opp {
                                                    // different enough to stay distinct
                                                    CtrlPoint::Exact(c_opp_ex) if c_opp_ex.distance_to(pp.p) > SNAP_VERT_RADIUS_SQR => (),

                                                    _ => *c_opp = CtrlPoint::Corner,
                                                }
                                            } else if let CtrlPoint::Exact(c_opp_ex) = &c_opp {
                                                // snap to smooth
                                                let c_self_smooth = c_opp_ex.reflected_over(pp.p);
                                                if c_self_ex.distance_sqr_to(c_self_smooth) <= SNAP_VERT_RADIUS_SQR {
                                                    *c_self = CtrlPoint::Smooth;
                                                }
                                            }
                                        }
                                    }
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
                        let Layer::Group(_group) = &mut *group_layer.borrow_mut() else { panic!("GroupHover must reference a Group layer") };
                        todo!()
                    }
                    Hover::Path(PathHover { path_layer, ref region }) => {
                        let Layer::Path(path) = &mut *path_layer.borrow_mut() else { panic!("PathHover must reference a Path layer") };
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
                                    PointPart::CtrlIn => pp.c_in = CtrlPoint::Exact(mouse_world_pos),
                                    PointPart::Point => pp.set_point(mouse_world_pos),
                                    PointPart::CtrlOut => pp.c_out = CtrlPoint::Exact(mouse_world_pos),
                                }
                            }
                        }
                    }
                    Hover::Raster(RasterHover { raster_layer, region: ref _region }) => {
                        let Layer::Raster(_raster) = &mut *raster_layer.borrow_mut() else { panic!("RasterHover must reference a Raster layer") };
                        todo!()
                    }
                }
            }

            Some(HoverOrDrag { is_dragging: false, .. }) | None => {
                self.hovered = document.layers
                    .tree_iter(
                        LayerIterDir::ForeToBack,
                        |group| !group.settings.is_hidden && !group.settings.is_locked)
                    .find_map(|(layer_rc, _depth)| -> Option<Hover> {
                        match &mut *layer_rc.borrow_mut() {
                            Layer::Group(_group) => {
                                // todo
                                None
                            },
                            Layer::Path(path) => {
                                path.points.iter()
                                    .enumerate()
                                    .find_map(|(i, point)| {
                                        ((point.p - mouse_world_pos).length_sqr() <= HOVER_RADIUS_SQR)
                                            .then(|| Hover::path_vert(layer_rc.clone(), i, PointPart::Point))
                                            .or_else(||
                                                match point.c_in {
                                                    CtrlPoint::Exact(c_in) => Some(c_in),
                                                    CtrlPoint::Smooth => Some(point.c_in.calculate(&point.p, &point.c_out)),
                                                    CtrlPoint::Corner => None,
                                                }.and_then(|c_in|
                                                    ((c_in - mouse_world_pos).length_sqr() <= HOVER_RADIUS_SQR)
                                                        .then(|| Hover::path_vert(layer_rc.clone(), i, PointPart::CtrlIn))
                                                )
                                            )
                                            .or_else(||
                                                match point.c_out {
                                                    CtrlPoint::Exact(c_out) => Some(c_out),
                                                    CtrlPoint::Smooth => Some(point.c_out.calculate(&point.p, &point.c_in)),
                                                    CtrlPoint::Corner => None,
                                                }.and_then(|c_out|
                                                    ((c_out - mouse_world_pos).length_sqr() <= HOVER_RADIUS_SQR)
                                                        .then(|| Hover::path_vert(layer_rc.clone(), i, PointPart::CtrlOut))
                                                )
                                            )
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

    fn draw(&self, d: &mut impl RaylibDraw, _document: &Document, _mouse_world_pos: Vector2) {
        if let Some(HoverOrDrag { hover, is_dragging }) = &self.hovered {
            match hover {
                Hover::Group(GroupHover { group_layer }) => {
                    let Layer::Group(_group) = &*group_layer.borrow() else { panic!("GroupHover must reference a Group layer") };
                    todo!()
                }
                Hover::Path(PathHover { path_layer, region }) => {
                    let Layer::Path(path) = &*path_layer.borrow() else { panic!("PathHover must reference a Path layer") };
                    match region {
                        PathHoverRegion::Fill => todo!(),

                        PathHoverRegion::Edge => todo!(),

                        PathHoverRegion::Vert { point, part } => {
                            const C_IN_COLOR: Color = Color::FORESTGREEN;
                            const P_COLOR: Color = Color::DODGERBLUE;
                            const C_OUT_COLOR: Color = Color::ORANGE;
                            let pp = &path.points[*point];
                            if matches!(part, PointPart::Point) {
                                d.draw_circle_v(pp.p, HOVER_RADIUS, P_COLOR);
                            } else {
                                let (c_self, c_opp, color) = match part {
                                    PointPart::CtrlIn  => (&pp.c_in, &pp.c_out, C_IN_COLOR),
                                    PointPart::CtrlOut => (&pp.c_out, &pp.c_in, C_OUT_COLOR),
                                    PointPart::Point => unreachable!(),
                                };
                                match c_self {
                                    &CtrlPoint::Exact(mut c_self) => {
                                        if *is_dragging {
                                            // preview snap
                                            if c_self.distance_sqr_to(pp.p) <= SNAP_VERT_RADIUS_SQR {
                                                c_self = pp.p;
                                            } else if let CtrlPoint::Exact(c_opp) = c_opp {
                                                let c_self_smooth = c_opp.reflected_over(pp.p);
                                                if c_self.distance_sqr_to(c_self_smooth) <= SNAP_VERT_RADIUS_SQR {
                                                    c_self = c_self_smooth;
                                                }
                                                d.draw_line_v(pp.p, c_self_smooth, color.alpha(0.5));
                                                d.draw_circle_v(c_self_smooth, SNAP_VERT_RADIUS, color.alpha(0.5));
                                            }
                                        }
                                        d.draw_circle_v(c_self, if !is_dragging { HOVER_RADIUS } else { SNAP_VERT_RADIUS }, color);
                                    }

                                    CtrlPoint::Smooth => {
                                        let CtrlPoint::Exact(c_opp) = c_opp else { panic!("should not hover smooth opposite of non-exact") };
                                        let c_self_smooth = c_opp.reflected_over(pp.p);
                                        d.draw_circle_v(c_self_smooth, if !is_dragging { HOVER_RADIUS } else { SNAP_VERT_RADIUS }, color);
                                    }

                                    CtrlPoint::Corner => panic!("should not hover corner"),
                                }
                            }
                        }
                    }
                }
                Hover::Raster(RasterHover { raster_layer, region }) => {
                    let Layer::Raster(_raster) = &*raster_layer.borrow() else { panic!("RasterHover must reference a Raster layer") };
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
