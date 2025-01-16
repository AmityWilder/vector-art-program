use std::{cell::RefCell, rc::Rc};
use rand::{distributions::Uniform, prelude::*};
use raylib::prelude::*;
use crate::{layer::{Layer, LayerContent}, Document, VectorPath};

pub struct DirectSelection {
    pub selection: Vec<Rc<RefCell<Layer>>>,
}

impl DirectSelection {
    pub fn new() -> Self {
        Self {
            selection: Vec::new(),
        }
    }
}

pub struct Pen {
    /// If [`Some`], continue seleted.
    /// If [`None`], find a hovered path or create a new path upon clicking.
    /// Must be a `VectorPath` layer
    pub current_path: Option<Rc<RefCell<Layer>>>,

    /// [`Some`] while dragging, [`None`] otherwise.
    pub current_anchor: Option<Vector2>,
}

pub enum Tool {
    DirectSelection(DirectSelection),
    Pen(Pen),
}

impl Default for Tool {
    fn default() -> Self {
        Self::DirectSelection(DirectSelection::new())
    }
}

impl Tool {
    pub fn switch_to_direct_selection(&mut self) {
        *self = Self::DirectSelection(DirectSelection {
            selection: Vec::new(),
        })
    }

    pub fn switch_to_pen(&mut self) {
        *self = Self::Pen(Pen {
            current_path: match self {
                Tool::DirectSelection(DirectSelection { selection }) if selection.len() == 1 => {
                    match &*selection[0].borrow_mut() {
                        LayerContent::Path(path) => Some(path.clone()),
                        _ => None,
                    }
                }
                _ => None,
            },
            current_anchor: None,
        })
    }

    pub fn tick<'a>(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2, rng: &mut ThreadRng) {
        let uniform_u8 = Uniform::new_inclusive(0u8, 255u8);
        match self {
            Tool::DirectSelection(_) => {
                // todo
            }

            Tool::Pen(Pen { current_path, current_anchor }) => {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    if current_path.is_none() {
                        // create a new path
                        let layer = Rc::new(RefCell::new(Layer::new(
                            format!("layer {}", document.layers.len()),
                            Color::new(
                                uniform_u8.sample(rng),
                                uniform_u8.sample(rng),
                                uniform_u8.sample(rng),
                                255,
                            ),
                            LayerContent::Path(VectorPath::new()),
                        )));
                        document.current_layer = Some(layer.clone());
                        document.layers.push(layer);
                        *current_path = Some(path);
                    }
                    *current_anchor = Some(mouse_world_pos);
                } else if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                    if let Some(current_path) = current_path.as_mut() {
                        let mut path = current_path.borrow_mut();
                        if let Some(anchor) = current_anchor.take() {
                            path.points.push((
                                anchor * 2.0 - mouse_world_pos, // x - (a - x) = 2x - a
                                anchor,
                                mouse_world_pos,
                            ));
                        }
                    } else {
                        println!("warning: `current_path` should have been set when mouse was pressed");
                    }
                }
            }
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw, document: &Document, mouse_world_pos: Vector2) {
        match self {
            Tool::Pen(Pen { current_path: Some(current_path), current_anchor }) => {
                let c_out = mouse_world_pos;
                let path = current_path.borrow();
                let layer_color = document.current_layer.as_ref().expect("cannot draw without active layer").borrow().color;
                match current_anchor {
                    &Some(p) => {
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

            _ => (),
        }
    }
}
