use raylib::prelude::*;
use crate::{layer::{Layer, LayerType, StrongLayer}, Document};

use super::ToolType;

pub struct Pen {
    /// If [`Some`], continue seleted.
    /// If [`None`], find a hovered path or create a new path upon clicking.
    /// Must be a `VectorPath` layer.
    /// If there is a layer, it must not die before the pen dies.
    pub target: Option<StrongLayer>,

    /// [`Some`] while dragging, [`None`] otherwise.
    pub current_anchor: Option<Vector2>,

    layer_color: Color,
}

impl Pen {
    pub fn new() -> Self {
        Self {
            target: None,
            current_anchor: None,
            layer_color: Color::default(),
        }
    }
}

impl ToolType for Pen {
    fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if self.target.is_none() {
                // create a new path
                let new_path = document.create_path(None, None);
                self.layer_color = new_path.borrow().settings().color;
                self.target = Some(new_path);
            }
            self.current_anchor = Some(mouse_world_pos);
        }

        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            if let Some(anchor) = self.current_anchor.take() {
                let mut target = self.target.as_ref().expect("`target` should have been set when mouse was pressed").borrow_mut();
                if let Layer::Path(path) = &mut *target {
                    path.points.push((
                        anchor * 2.0 - mouse_world_pos, // x - (a - x) = 2x - a
                        anchor,
                        mouse_world_pos,
                    ));
                }
            } else {
                println!("warning: pen was released without having been pressed");
            }
        }
    }

    fn draw(&self, d: &mut impl RaylibDraw, _document: &Document, mouse_world_pos: Vector2) {
        if let Some(target) = self.target.as_ref() {
            let target = target.borrow();
            if let Layer::Path(path) = &*target {
                let c_out = mouse_world_pos;
                let layer_color = self.layer_color;
                match self.current_anchor {
                    Some(p) => {
                        let c_in = p * 2.0 - c_out;
                        if let Some((_, p_last, c_out_last)) = path.points.last() {
                            d.draw_spline_segment_bezier_cubic(
                                *p_last,
                                *c_out_last,
                                c_in,
                                p,
                                1.0,
                                layer_color,
                            );
                        }
                        d.draw_line_v(p, c_out, layer_color);
                        d.draw_line_v(p, p * 2.0 - c_out, layer_color);
                        d.draw_circle_v(c_in, 3.0, layer_color);
                        d.draw_circle_v(p, 5.0, layer_color);
                        d.draw_circle_v(c_out, 3.0, layer_color);
                    }
                    None => {
                        if let Some((_, p_last, c_out_last)) = path.points.last() {
                            d.draw_spline_segment_bezier_cubic(
                                *p_last,
                                *c_out_last,
                                c_out,
                                c_out,
                                1.0,
                                layer_color,
                            );
                        }
                        d.draw_circle_v(c_out, 3.0, layer_color);
                    }
                }
            }
        }
    }
}
