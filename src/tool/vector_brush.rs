use raylib::prelude::*;
use amylib::rc::prelude::*;
use amymath::prelude::{*, Vector2};
use crate::{appearance::Appearance, document::Document, editor::Editor, layer::LayerType, shaders::ShaderTable, vector_path::{path_point::{Ctrl, Ctrl1, Ctrl2, PathPoint}, DrawPathPoint, VectorPath}};
use super::{point_selection::SNAP_VERT_RADIUS_SQR, ToolType};

use MouseButton::MOUSE_BUTTON_LEFT;

pub struct InactiveVectorBrush(pub(super) Option<StrongMut<VectorPath>>);

impl InactiveVectorBrush {
    fn tick(
        rl: &mut RaylibHandle,
        current_appearance: &Appearance,
        document: &mut Document,
    ) -> Option<ActiveVectorBrush> {
        if rl.is_mouse_button_pressed(MOUSE_BUTTON_LEFT) {
            // create a new path
            return Some(ActiveVectorBrush {
                target: document.create_path(None, None, current_appearance.clone()).clone_mut(),
                signal: PathSignal::default(),
            })
        }
        None
    }
}

pub struct ActiveVectorBrush {
    pub(super) target: StrongMut<VectorPath>,

    signal: PathSignal,
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
        let distance_sqr = delta.magnitude_sqr();

        if distance_sqr > MIN_DISTANCE_SQR {
            change_type = ChangeType::Position;

            if self.last_straight != last_changed {
                let last_straight = self.last_straight;
                let delta_prev = last_changed - last_straight;
                let distance_sqr_prev = delta_prev.magnitude_sqr();
                assert!(distance_sqr_prev.is_normal() && distance_sqr_prev > 0.0, "delta_prev: {delta_prev:?}");
                let adj_magnitude = (delta_prev / distance_sqr_prev.sqrt()).dot(delta);
                let opp_magnitude_sqr = distance_sqr - adj_magnitude * adj_magnitude;

                if adj_magnitude < 0.0 {
                    change_type = ChangeType::Curvature;

                    self.last_curved = p;
                    self.last_straight = p;
                } else if opp_magnitude_sqr > MIN_OPP_LENGTH_SQR {
                    change_type = ChangeType::Direction;

                    if self.last_curved != last_straight {
                        let last_curved = self.last_curved;
                        let delta_prev_prev = last_straight - last_curved;
                        let distance_sqr_prev_prev = delta_prev_prev.magnitude_sqr();
                        let adj_magnitude_prev = (delta_prev_prev / distance_sqr_prev_prev.sqrt()).dot(delta_prev);
                        let opp_magnitude_sqr_prev = distance_sqr_prev - adj_magnitude_prev * adj_magnitude_prev;

                        if (opp_magnitude_sqr - opp_magnitude_sqr_prev).abs() > MIN_OPP_LENGTH_SQR_CHANGE {
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

    // At high speeds, a point can be made with a zero-magnitude control, creating what looks like a distinct path.
    // This function fixes that.
    fn merge_confirmed_verts(path: &mut VectorPath) {
        // join points confirmed to be no longer editing
        if let Some(idx) = path.curve.points.len().checked_sub(3) && path.curve.points[idx].p.distance_sqr(path.curve.points[idx + 1].p) < SNAP_VERT_RADIUS_SQR {
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
            let speed_in  = prev.distance(curr);
            let speed_out = curr.distance(next);
            let t_hat = prev.delta(next).normalized();
            let c_out = curr + t_hat * speed_out / 3.0;
            {
                let curr_c = &mut path.curve.points[idx + 1].c;
                *curr_c = Some(Ctrl1 {
                    c1: (Ctrl::Out, c_out),
                    c2: Some(Ctrl2::Mirror(speed_in / 3.0)),
                });
            }
        } else if let Some(idx) = path.curve.points.len().checked_sub(2) {
            let (prev, curr) = (path.curve.points[idx].p, path.curve.points[idx + 1].p);
            let speed = prev.distance(curr);
            let t_hat = prev.delta(curr).normalized();
            {
                let c_out = prev + t_hat * speed / 3.0;
                let prev_c = &mut path.curve.points[idx].c;
                *prev_c = Some(Ctrl1 {
                    c1: (Ctrl::Out, c_out),
                    c2: Some(Ctrl2::Reflect),
                });
            }
            {
                let c_out = curr + t_hat * speed / 3.0;
                let curr_c = &mut path.curve.points[idx + 1].c;
                *curr_c = Some(Ctrl1 {
                    c1: (Ctrl::Out, c_out),
                    c2: Some(Ctrl2::Reflect),
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
        mouse_world_pos: Vector2,
    ) -> InactiveVectorBrush {
        {
            let mut path = self.target.write();
            path.curve.points.push_back(PathPoint { p: mouse_world_pos, c: None });
            if path.curve.points.len() >= 2 {
                let first = path.curve.points.front().expect("len >= 2 should guarantee 2 points").p;
                let last  = path.curve.points.back ().expect("len >= 2 should guarantee 2 points").p;
                if first.distance_sqr(last) <= SNAP_VERT_RADIUS_SQR {
                    path.curve.is_closed = true;
                }
            }
        }
        InactiveVectorBrush(Some(self.target.clone_mut()))
    }

    /// Returns true if still active, false if finished
    fn tick(
        &mut self,
        rl: &mut RaylibHandle,
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
            Some(self.finish_path(mouse_world_pos))
        } else { None }
    }

    fn draw(&self, d: &mut impl RaylibDraw, editor: &Editor, _shader_table: &ShaderTable) {
        let px_world_size = editor.document.camera.zoom.recip();
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

    pub fn target(&self) -> Option<Strong<VectorPath>> {
        if let Self::Inactive(InactiveVectorBrush(Some(target))) | Self::Active(ActiveVectorBrush { target, .. }) = self {
            return Some(target.clone_ref());
        }
        None
    }

    pub fn target_mut(&mut self) -> Option<StrongMut<VectorPath>> {
        if let Self::Inactive(InactiveVectorBrush(Some(target))) | Self::Active(ActiveVectorBrush { target, .. }) = self {
            return Some(target.clone_mut());
        }
        None
    }
}

impl ToolType for VectorBrush {
    fn tick(
        &mut self,
        rl: &mut RaylibHandle,
        _thread: &RaylibThread,
        current_appearance: &mut Appearance,
        document: &mut Document,
        _scratch_rtex: &mut Vec<RenderTexture2D>,
        mouse_world_pos: Vector2,
        _px_world_size: f32,
    ) {
        if let VectorBrush::Inactive(_) = self {
            if let Some(active_brush) = InactiveVectorBrush::tick(rl, current_appearance, document) {
                // todo: only do this if at a new depth
                // scratch_rtex.push(rl.load_render_texture(thread, rl.get_screen_width() as u32, rl.get_screen_height() as u32).unwrap());
                *self = Self::Active(active_brush);
            }
        }

        if let VectorBrush::Active(brush) = self {
            if let Some(inactive_brush) = brush.tick(rl, mouse_world_pos) {
                *self = Self::Inactive(inactive_brush);
            }
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, editor: &Editor, shader_table: &ShaderTable, px_world_size: f32, _viewport: &Rect2, #[cfg(dev)] _mouse_world_pos: Vector2) {
        const DRAW_DEBUG: bool = false;
        if let VectorBrush::Active(brush) = self {
            brush.draw(d, editor, shader_table);

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
