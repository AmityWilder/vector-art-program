use std::{collections::VecDeque, fmt};
use raylib::prelude::*;
use amylib::rc::prelude::*;
use amymath::prelude::*;
use crate::{layer::LayerType, shaders::ShaderTable, vector_path::{path_point::{Ctrl, Ctrl1, Ctrl2, PathPoint}, VectorPath, DrawPathPoint}, Change, Document};
use super::ToolType;

use MouseButton::MOUSE_BUTTON_LEFT;

struct BrushAction {
    target: StrongMut<VectorPath>,
    stroke: VecDeque<PathPoint>,
}

impl fmt::Debug for BrushAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BrushAction").finish()
    }
}

impl Change for BrushAction {
    fn redo(&mut self, _document: &mut Document) -> Result<(), String> {
        self.target.write().curve.points.clone_from(&self.stroke);
        Ok(())
    }

    fn undo(&mut self, _document: &mut Document) -> Result<(), String> {
        self.target.write().curve.points.clear();
        Ok(())
    }
}

pub struct InactiveVectorBrush(pub(super) Option<StrongMut<VectorPath>>);

impl InactiveVectorBrush {
    fn tick(
        rl: &mut RaylibHandle,
        document: &mut Document,
    ) -> Option<ActiveVectorBrush> {
        if rl.is_mouse_button_pressed(MOUSE_BUTTON_LEFT) {
            // create a new path
            return Some(ActiveVectorBrush {
                target: document.create_path(None, None).clone_mut(),
                signal: PathSignal::default(),
                last_failing: None,
            })
        }
        None
    }
}

pub struct ActiveVectorBrush {
    /// If [`Some`], continue seleted.
    /// If [`None`], find a hovered path or create a new path upon clicking.
    /// Must be a `VectorPath` layer.
    /// If there is a layer, it must not die before the pen dies.
    pub(super) target: StrongMut<VectorPath>,

    signal: PathSignal,

    /// The position the last time its dot product was 1
    last_failing: Option<Vector2>,
}

const MIN_DISTANCE: f32 = 2.0;
const MIN_DISTANCE_SQR: f32 = MIN_DISTANCE * MIN_DISTANCE;
const MIN_OPP_LENGTH: f32 = 0.5;
const MIN_OPP_LENGTH_SQR: f32 = MIN_OPP_LENGTH * MIN_OPP_LENGTH;
const MIN_OPP_LENGTH_SQR_CHANGE: f32 = 0.25;
const IS_CURVATURE_SUPPORTED: bool = false;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ChangeType {
    None,
    Position,
    Direction,
    Curvature,
}

#[derive(Debug, Default)]
struct PathSignal {
    last_changed:  Vector2,
    last_straight: Vector2,
    last_curved:   Vector2,
}

impl PathSignal {
    fn test(&mut self, p: Vector2) -> ChangeType {
        let mut change_type = ChangeType::None;
        let last_changed = self.last_changed;
        let delta = p - last_changed;
        let distance_sqr = delta.length_sqr();

        if distance_sqr > MIN_DISTANCE_SQR {
            change_type = ChangeType::Position;

            if self.last_straight != last_changed {
                let last_straight = self.last_straight;
                let delta_prev = last_changed - last_straight;
                let distance_sqr_prev = delta_prev.length_sqr();
                assert!(distance_sqr_prev.is_normal() && distance_sqr_prev > 0.0, "delta_prev: {delta_prev:?}");
                let adj_length = (delta_prev / distance_sqr_prev.sqrt()).dot(delta);
                let opp_length_sqr = distance_sqr - adj_length * adj_length;

                if adj_length < 0.0 {
                    change_type = ChangeType::Curvature;

                    self.last_curved = p;
                    self.last_straight = p;
                } else if opp_length_sqr > MIN_OPP_LENGTH_SQR {
                    change_type = ChangeType::Direction;

                    if self.last_curved != last_straight {
                        let last_curved = self.last_curved;
                        let delta_prev_prev = last_straight - last_curved;
                        let distance_sqr_prev_prev = delta_prev_prev.length_sqr();
                        let adj_length_prev = (delta_prev_prev / distance_sqr_prev_prev.sqrt()).dot(delta_prev);
                        let opp_length_sqr_prev = distance_sqr_prev - adj_length_prev * adj_length_prev;

                        if (opp_length_sqr - opp_length_sqr_prev).abs() > MIN_OPP_LENGTH_SQR_CHANGE {
                            change_type = ChangeType::Curvature;

                            self.last_curved = p;
                        }
                    }

                    self.last_straight = p;
                }
            }

            self.last_changed = p;
        }

        change_type
    }
}

impl ActiveVectorBrush {
    fn begin_path(
        &mut self,
        rl: &mut RaylibHandle,
        mouse_world_pos: Vector2,
    ) {
        if rl.is_mouse_button_pressed(MOUSE_BUTTON_LEFT) {
            self.signal.last_changed  = mouse_world_pos;
            self.signal.last_straight = mouse_world_pos;
            self.signal.last_curved   = mouse_world_pos;
            let mut path = self.target.write();
            for _ in 0..2 {
                path.curve.points.push_back(PathPoint { p: mouse_world_pos, c: None });
            }
        }
    }

    // At high speeds, a point can be made with a zero-length control, creating what looks like a distinct path.
    // This function fixes that.
    fn merge_confirmed_verts(path: &mut VectorPath) {
        // join points confirmed to be no longer editing
        if let Some(idx) = path.curve.points.len().checked_sub(3) && path.curve.points[idx].p.distance_sqr_to(path.curve.points[idx + 1].p) < 0.001 {
            let b = path.curve.points.remove(idx + 1).expect("checked sub should ensure element existence");
            let a = &mut path.curve.points[idx];
            println!("merging points\n  {a:?}\n  {b:?}");
            if let Some((c_a, c_b)) = a.c.as_mut().zip(b.c) {
                assert_eq!((c_a.c1.0, c_b.c1.0), (Ctrl::Out, Ctrl::Out), "brush should be producing Out ctrl only");
                c_a.c1.1 = a.p + c_b.c1.1 - b.p; // merge output velocity, keep input velocity
            }
            println!("result:\n  {a:?}");
        }
    }

    fn update_path(path: &mut VectorPath, new_pos: Vector2) {
        if let Some(back) = path.curve.points.back_mut() {
            back.p = new_pos;
        }

        if let Some(idx) = path.curve.points.len().checked_sub(3) {
            let (prev, curr, next) = (path.curve.points[idx].p, path.curve.points[idx + 1].p, path.curve.points[idx + 2].p);
            let speed_in  = (curr - prev).length();
            let speed_out = (next - curr).length();
            let t_hat = (next - prev).normalized();
            let c_out = curr + t_hat * speed_out / 3.0;
            {
                let curr = &mut path.curve.points[idx + 1].c;
                *curr = Some(Ctrl1 {
                    c1: (Ctrl::Out, c_out),
                    c2: Some(Ctrl2::Mirror(speed_in / 3.0)),
                });
            }
        }

        Self::merge_confirmed_verts(path);
    }

    fn continue_path(&mut self, mouse_world_pos: Vector2) {
        let new_pos = mouse_world_pos;

        let change_type = self.signal.test(new_pos);

        Self::update_path(&mut self.target.write(), new_pos);

        if matches!(change_type, ChangeType::Curvature) {
            self.target.write().curve.points.push_back(PathPoint { p: self.signal.last_curved, c: None });
        }
    }

    fn finish_path(
        &mut self,
        _document: &mut Document,
        mouse_world_pos: Vector2,
    ) -> InactiveVectorBrush {
        let _stroke = {
            let mut path = self.target.write();

            path.curve.points.push_back(PathPoint { p: mouse_world_pos, c: None });

            path.curve.points.clone()
        };
        // todo: rework undo/redo

        // document.push_change(
        //     Box::new(BrushAction {
        //         target: self.target.clone_mut(),
        //         stroke,
        //     })
        // );
        InactiveVectorBrush(Some(self.target.clone_mut()))
    }

    /// Returns true if still active, false if finished
    fn tick(
        &mut self,
        rl: &mut RaylibHandle,
        document: &mut Document,
        mouse_world_pos: Vector2,
    ) -> Option<InactiveVectorBrush> {
        if rl.is_mouse_button_pressed(MOUSE_BUTTON_LEFT) {
            self.begin_path(rl, mouse_world_pos);
        }

        if rl.is_mouse_button_down(MOUSE_BUTTON_LEFT) {
            self.continue_path(mouse_world_pos);
        }

        // stroke complete
        if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
            Some(self.finish_path(document, mouse_world_pos))
        } else { None }
    }

    fn draw(&self, d: &mut impl RaylibDraw, document: &Document, _shader_table: &ShaderTable) {
        let px_world_size = document.camera.zoom.recip();
        let path = self.target.read();
        path.draw_selected(d, px_world_size);
        let color = path.settings.color;
        for pp in &path.curve.points {
            d.draw_path_point(pp, px_world_size, color, false, true, true);
        }
    }
}

pub enum VectorBrush {
    Inactive(InactiveVectorBrush),
    Active(ActiveVectorBrush),
}

impl VectorBrush {
    pub fn new() -> Self {
        Self::Inactive(InactiveVectorBrush(None))
    }
}

impl ToolType for VectorBrush {
    fn tick(&mut self, rl: &mut RaylibHandle, _thread: &RaylibThread, document: &mut Document, mouse_world_pos: Vector2) {
        if let VectorBrush::Inactive(_) = self {
            if let Some(active_brush) = InactiveVectorBrush::tick(rl, document) {
                *self = Self::Active(active_brush);
            }
        }

        if let VectorBrush::Active(brush) = self {
            if let Some(inactive_brush) = brush.tick(rl, document, mouse_world_pos) {
                *self = Self::Inactive(inactive_brush);
            }
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, document: &Document, shader_table: &ShaderTable, px_world_size: f32, _viewport: &Rect2, #[cfg(dev)] _mouse_world_pos: Vector2) {
        const DRAW_DEBUG: bool = false;
        if let VectorBrush::Active(brush) = self {
            brush.draw(d, document, shader_table);

            if DRAW_DEBUG {
                let last_changed = brush.signal.last_changed;
                let last_straight = brush.signal.last_straight;
                let last_curved = brush.signal.last_curved;

                d.draw_pixel_v(last_changed, Color::BLUE);
                d.draw_ring(last_changed, MIN_DISTANCE, MIN_DISTANCE + px_world_size, 0.0, 360.0, 36, Color::BLUE);

                d.draw_pixel_v(last_straight, Color::RED);
                let straight_direction = (last_changed - last_straight).normalized();
                let perp_a = Vector2 { x: -straight_direction.y, y:  straight_direction.x };
                let perp_b = Vector2 { x:  straight_direction.y, y: -straight_direction.x };
                d.draw_line_v(last_straight + perp_a * MIN_OPP_LENGTH, last_changed + perp_a * MIN_OPP_LENGTH, Color::RED);
                d.draw_line_v(last_straight + perp_b * MIN_OPP_LENGTH, last_changed + perp_b * MIN_OPP_LENGTH, Color::RED);

                d.draw_pixel_v(last_curved, Color::GREEN);
            }
        }
    }
}
