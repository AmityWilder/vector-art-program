use std::{cell::RefCell, rc::Rc};
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

    pub fn tick(&mut self, _rl: &mut RaylibHandle, _document: &mut Document, _mouse_world_pos: Vector2) {
        // todo
    }

    pub fn draw(&self, _d: &mut impl RaylibDraw, _document: &Document, _mouse_world_pos: Vector2) {
        // todo
    }
}

pub struct Pen {
    /// If [`Some`], continue seleted.
    /// If [`None`], find a hovered path or create a new path upon clicking.
    /// Must be a `VectorPath` layer
    pub target: Option<Rc<RefCell<VectorPath>>>,

    /// [`Some`] while dragging, [`None`] otherwise.
    pub current_anchor: Option<Vector2>,
}

impl Pen {
    pub fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if self.target.is_none() {
                // create a new path
                let new_path = Rc::new(RefCell::new(VectorPath::new()));
                document.current_layer = Some(document.create_layer(None, None, LayerContent::Path(new_path.clone())));
                self.target = Some(new_path);
            }
            self.current_anchor = Some(mouse_world_pos);
        } else if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            if let Some(anchor) = self.current_anchor.take() {
                let mut target = self.target.as_ref()
                    .expect("`target` should have been set when mouse was pressed")
                    .borrow_mut();

                target.points.push((
                    anchor * 2.0 - mouse_world_pos, // x - (a - x) = 2x - a
                    anchor,
                    mouse_world_pos,
                ));
            } else {
                println!("warning: pen was released without having been pressed");
            }
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw, document: &Document, mouse_world_pos: Vector2) {
        if let Self { target: Some(current_path), current_anchor } = self {
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
    }
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
        match self {
            Tool::DirectSelection(direct_selection) => (),
            Tool::Pen(pen) => {
                *self = Self::DirectSelection(DirectSelection {
                    selection: pen.target,
                })
            },
        }
        *self = Self::DirectSelection(DirectSelection {
            selection: match self {
                Tool::DirectSelection(direct_selection) => direct_selection,
                Tool::Pen(pen) => todo!(),
            },
        })
    }

    pub fn switch_to_pen(&mut self) {
        *self = Self::Pen(Pen {
            target: match self {
                Tool::DirectSelection(DirectSelection { selection }) => {
                    match selection.len() {
                        1 => {
                            match &selection[0].borrow().content {
                                LayerContent::Path(path) => Some(path.clone()),
                                _ => None
                            }
                        }
                        _ => None,
                    }
                }
                _ => None,
            },
            current_anchor: None,
        })
    }

    pub fn tick(&mut self, rl: &mut RaylibHandle, document: &mut Document, mouse_world_pos: Vector2) {
        match self {
            Tool::DirectSelection(direct_selection) => direct_selection.tick(rl, document, mouse_world_pos),
            Tool::Pen(pen) => pen.tick(rl, document, mouse_world_pos),
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw, document: &Document, mouse_world_pos: Vector2) {
        match self {
            Tool::Pen(pen) => pen.draw(d, document, mouse_world_pos),
            _ => (),
        }
    }
}
