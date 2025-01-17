use raylib::prelude::*;
use crate::{layer::{tree::LayerIterDir, Layer, StrongLayer}, vector_path::path_point::PathPoint, Document};
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

pub struct DirectSelection {
    /// (path, point, cin vs p vs cout)
    hovered: Option<Hover>,
}

impl DirectSelection {
    pub fn new() -> Self {
        Self { hovered: None }
    }
}

pub const HOVER_RADIUS: f32 = 5.0;
pub const HOVER_RADIUS_SQR: f32 = HOVER_RADIUS * HOVER_RADIUS;

impl ToolType for DirectSelection {
    fn tick(&mut self, _rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2) {
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
                                if (point.p - mouse_world_pos).length_sqr() <= HOVER_RADIUS_SQR {
                                    Some(Hover::path_vert(layer_rc.clone(), i, PointPart::Point))
                                } else if (point.c_in - mouse_world_pos).length_sqr() <= HOVER_RADIUS_SQR {
                                    Some(Hover::path_vert(layer_rc.clone(), i, PointPart::CtrlIn))
                                } else if (point.c_out - mouse_world_pos).length_sqr() <= HOVER_RADIUS_SQR {
                                    Some(Hover::path_vert(layer_rc.clone(), i, PointPart::CtrlOut))
                                } else { None }
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
            });
    }

    fn draw(&self, d: &mut impl RaylibDraw, _document: &Document, _mouse_world_pos: Vector2) {
        if let Some(hover) = &self.hovered {
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

                        PathHoverRegion::Vert { point, part } => d.draw_circle_v(
                            (|pp: &PathPoint| match part {
                                PointPart::CtrlIn  => pp.c_in,
                                PointPart::Point   => pp.p,
                                PointPart::CtrlOut => pp.c_out,
                            })(&path.points[*point]),
                            HOVER_RADIUS,
                            Color::DODGERBLUE,
                        ),
                    }
                }
                Hover::Raster(RasterHover { raster_layer, region }) => {
                    let Layer::Raster(_raster) = &*raster_layer.borrow() else { panic!("RasterHover must reference a Raster layer") };
                    match region {
                        RasterHoverRegion::Object => todo!(),
                        RasterHoverRegion::Side { side } => match side {
                            Side::Top => todo!(),
                            Side::Right => todo!(),
                            Side::Bottom => todo!(),
                            Side::Left => todo!(),
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
