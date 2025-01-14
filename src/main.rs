use std::{cell::RefCell, collections::BTreeMap, rc::Rc};
use raylib::prelude::*;
// use rand::prelude::*;

// pub trait RenderHeight {
//     fn get_render_height(&self) -> i32 {
//         unsafe { ffi::GetRenderHeight() }
//     }
// }
// impl RenderHeight for RaylibHandle {}

fn mix(c0: &Color, c1: &Color, amount: f32) -> Color {
    Color {
        r: lerp(c0.r as f32, c1.r as f32, amount) as u8,
        g: lerp(c0.g as f32, c1.g as f32, amount) as u8,
        b: lerp(c0.b as f32, c1.b as f32, amount) as u8,
        a: lerp(c0.a as f32, c1.a as f32, amount) as u8,
    }
}

pub struct Blending {
    pub opacity: f32,
    pub mode: BlendMode,
}

impl Default for Blending {
    fn default() -> Self {
        Self {
            opacity: 1.0,
            mode: BlendMode::BLEND_ALPHA,
        }
    }
}

pub mod gradient {
    use super::*;

    pub struct Control {
        pos: u8,
        color: Color,
    }

    pub struct Ramp {
        /// Should be sorted and unique by `pos`
        colors: Vec<Control>,
    }

    impl Ramp {
        pub fn clean(&mut self) {
            self.colors.sort_by_key(|x| x.pos);
            self.colors.dedup_by_key(|x| x.pos);
        }

        pub fn color_at(&self, t: f32) -> Color {
            let pos = u8::try_from((t * 255.0).floor() as isize).expect("t must be between 0 and 1");
            let mut lower: &Control = &self.colors[0];
            let mut upper: &Control = &self.colors[0];
            // todo: use binary search
            for ctrl in &self.colors {
                if ctrl.pos <= pos && ctrl.pos > lower.pos {
                    lower = ctrl;
                }
                if ctrl.pos > pos && ctrl.pos < upper.pos {
                    upper = ctrl;
                }
            }
            let t = (t - lower.pos as f32) / (upper.pos as f32 - lower.pos as f32); // normalize
            mix(&lower.color, &upper.color, t)
        }
    }
}
use gradient::Ramp as GradientRamp;

pub mod stroke {
    use super::*;

    pub enum GradientStyle {
        /// Parallel to the path
        Along,
        /// Perpendicular to the path
        Across,
        /// Gradient ignores the path and acts like a linear fill
        Within,
    }

    pub struct Gradient {
        pub ramp: GradientRamp,
        pub style: GradientStyle,
    }

    pub enum Pattern {
        Solid(Color),
        Gradient {
            ramp: GradientRamp,
            style: GradientStyle,
        },
    }

    pub enum Align {
        Inside,
        Middle,
        Outside,
    }

    pub enum WidthProfile {
        Constant(f32),
        Variable(BTreeMap<f32, f32>),
    }

    pub struct Stroke {
        pub blend: Blending,
        pub pattern: Pattern,
        pub thick: WidthProfile,
        pub align: Align,
    }

    impl Default for Stroke {
        fn default() -> Self {
            Self {
                blend: Blending::default(),
                pattern: Pattern::Solid(Color::BLACK),
                thick: WidthProfile::Constant(1.0),
                align: Align::Middle,
            }
        }
    }
}
use raylib::prelude::RaylibDraw;
use stroke::Stroke;

pub mod fill {
    use super::*;

    pub enum GradientStyle {
        Linear,
        Radial,
    }

    pub enum Pattern {
        Solid(Color),
        Gradient {
            ramp: GradientRamp,
            style: GradientStyle,
        },
    }

    pub struct Fill {
        pub blend: Blending,
        pub pattern: Pattern,
    }

    impl Default for Fill {
        fn default() -> Self {
            Self {
                blend: Blending::default(),
                pattern: Pattern::Solid(Color::WHITE),
            }
        }
    }
}
use fill::Fill;

pub enum StyleItem {
    Stroke(Stroke),
    Fill(Fill),
}

pub struct Appearance {
    pub items: Vec<StyleItem>,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            items: vec![
                StyleItem::Fill(Fill::default()),
                StyleItem::Stroke(Stroke::default()),
            ],
        }
    }
}

impl Appearance {
    pub fn new(items: Vec<StyleItem>) -> Self {
        Self {
            items,
        }
    }
}

pub struct VectorPath {
    /// p1, c2, c3, p4, c5, c6...
    pub points: Vec<(Vector2, Vector2, Vector2)>,
    pub appearance: Appearance,
}

impl VectorPath {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            appearance: Appearance::default(),
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        for window in self.points.windows(2) {
            if let [(_c1_in, p1, c1_out), (c2_in, p2, _c2_out)] = window {
                d.draw_spline_segment_bezier_cubic(*p1, *c1_out, *c2_in, *p2, 1.0, Color::BLUEVIOLET);
            }
        }
        for (c_in, p, c_out) in &self.points {
            d.draw_line_v(c_in, p, Color::BLUEVIOLET);
            d.draw_line_v(p, c_out, Color::BLUEVIOLET);
            d.draw_circle_v(c_in, 3.0, Color::BLUEVIOLET);
            d.draw_circle_v(c_out, 3.0, Color::BLUEVIOLET);
            d.draw_circle_v(p, 4.0, Color::BLUEVIOLET);
        }
    }
}

pub struct Bitmap {
    texture: Texture2D,
    source_rec: Rectangle,
    dest_rec: Rectangle,
    origin: Vector2,
    rotation: f32,
    tint: Color,
}

impl Bitmap {
    pub fn new(texture: Texture2D, position: Vector2) -> Self {
        let (width, height) = (texture.width as f32, texture.height as f32);
        Self {
            texture,
            source_rec: Rectangle::new(0.0, 0.0, width, height),
            dest_rec: Rectangle::new(position.x, position.y, width, height),
            origin: Vector2::new(width * 0.5, height * 0.5),
            rotation: 0.0,
            tint: Color::WHITE,
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        d.draw_texture_pro(&self.texture, self.source_rec, self.dest_rec, self.origin, self.rotation, self.tint);
    }
}

pub enum LayerItem {
    Group(Rc<Layer>),
    Path(Rc<RefCell<VectorPath>>),
    Bitmap(Bitmap),
}

pub struct Layer {
    pub name: String,
    pub is_hidden: bool,
    pub is_locked: bool,
    pub blend: Blending,
    pub bounds: Rectangle,
    pub items: Vec<Rc<RefCell<LayerItem>>>,
}

impl Layer {
    pub fn new(name: String) -> Self {
        Self {
            name,
            is_hidden: false,
            is_locked: false,
            blend: Blending::default(),
            bounds: Rectangle::default(),
            items: Vec::new(),
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        if !self.is_hidden {
            for item in &self.items {
                match &*item.borrow() {
                    LayerItem::Group(group) => group.draw(d),
                    LayerItem::Path(path) => path.borrow().draw(d),
                    LayerItem::Bitmap(bitmap) => bitmap.draw(d),
                }
            }
        }
    }
}

pub struct ArtBoard {
    pub name: String,
    pub rect: Rectangle,
}

impl ArtBoard {
    pub fn new(name: String, rect: Rectangle) -> Self {
        Self { name, rect }
    }
}

pub struct Document {
    pub title: String,
    pub camera: Camera2D,
    pub paper_color: Color,
    pub layers: Vec<Layer>,
    pub art_boards: Vec<ArtBoard>,
}

impl Document {
    pub fn new(title: String, width: f32, height: f32) -> Self {
        Self {
            title,
            camera: Camera2D {
                offset: Vector2::zero(),
                target: Vector2::zero(),
                rotation: 0.0,
                zoom: 1.0,
            },
            paper_color: Color::GRAY,
            layers: vec![Layer::new("layer 0".to_string())],
            art_boards: vec![ArtBoard::new("artboard 0".to_string(), rrect(0.0, 0.0, width, height))],
        }
    }
}

pub enum Tool {
    DirectSelection {
        selection: Vec<Rc<RefCell<LayerItem>>>,
    },
    Pen {
        /// If [`Some`], continue seleted.
        /// If [`None`], find a hovered path or create a new path upon clicking.
        current_path: Option<Rc<RefCell<VectorPath>>>,

        /// [`Some`] while dragging, [`None`] otherwise.
        current_anchor: Option<Vector2>,
    },
}

fn main() {
    let (mut rl, thread) = init()
        .title("Amity Vector Art")
        .size(1280, 720)
        .resizable()
        .build();

    rl.set_target_fps(60);

    // let mut rng = thread_rng();

    let background_color: Color = Color::new(32,32,32,255);

    let mut document = Document::new("untitled".to_string(), 256.0, 256.0);

    let mut mouse_screen_pos_prev = rl.get_mouse_position();
    let mut current_tool = Tool::DirectSelection { selection: Vec::new(), };
    let mut layers_panel = Rectangle::new(0.0, 0.0, 256.0, 0.0);
    layers_panel.x = rl.get_screen_width() as f32 - layers_panel.width;
    layers_panel.height = rl.get_screen_height() as f32;

    while !rl.window_should_close() {
        let mouse_screen_pos = rl.get_mouse_position();
        let mouse_screen_delta = mouse_screen_pos - mouse_screen_pos_prev;
        let mouse_world_pos = rl.get_screen_to_world2D(mouse_screen_pos, document.camera);

        if rl.is_window_resized() {
            layers_panel.x = rl.get_screen_width() as f32 - layers_panel.width;
            layers_panel.height = rl.get_screen_height() as f32;
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_MIDDLE) {
            document.camera.target -= mouse_screen_delta / document.camera.zoom;
        }

        document.camera.target += mouse_screen_delta / document.camera.zoom;
        document.camera.offset += mouse_screen_delta;

        if rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) {
            let scroll = rl.get_mouse_wheel_move();
            if scroll > 0.0 && document.camera.zoom < 8.0 {
                document.camera.zoom *= 2.0;
            } else if scroll < 0.0 && document.camera.zoom > 0.125 {
                document.camera.zoom *= 0.5;
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_V) {
            current_tool = Tool::DirectSelection {
                selection: Vec::new(),
            };
        } else if rl.is_key_pressed(KeyboardKey::KEY_P) {
            current_tool = Tool::Pen {
                current_path: match current_tool {
                    Tool::DirectSelection { selection } if selection.len() == 1 => {
                        match &*selection[0].borrow_mut() {
                            LayerItem::Path(path) => Some(path.clone()),
                            _ => None,
                        }
                    }
                    _ => None,
                },
                current_anchor: None,
            };
        }

        match &mut current_tool {
            Tool::DirectSelection { selection: _ } => {
                // todo
            }

            Tool::Pen { current_path, current_anchor } => {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    if current_path.is_none() {
                        // create a new path
                        let path = VectorPath::new();
                        let path = Rc::new(RefCell::new(path));
                        document.layers.push({
                            let mut layer = Layer::new(format!("layer {}", document.layers.len()));
                            layer.items.push(Rc::new(RefCell::new(LayerItem::Path(path.clone()))));
                            layer
                        });
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

        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(background_color);

            {
                let mut d = d.begin_mode2D(document.camera);

                // Artboards background
                for board in &document.art_boards {
                    d.draw_rectangle_rec(board.rect, document.paper_color);
                }

                for layer in &document.layers {
                    layer.draw(&mut d);
                }
                if let Tool::Pen { current_path: Some(current_path), current_anchor } = &current_tool {
                    let c_out = mouse_world_pos;
                    let path = current_path.borrow();
                    match current_anchor {
                        &Some(p) => {
                            let c_in = p * 2.0 - c_out;
                            if let Some((_, p_last, c_out_last)) = path.points.last() {
                                d.draw_spline_segment_bezier_cubic(
                                    *p_last,
                                    *c_out_last,
                                    c_in,
                                    p,
                                    1.0, Color::BLUEVIOLET);
                            }
                            d.draw_line_v(p, c_out, Color::BLUEVIOLET);
                            d.draw_line_v(p, p * 2.0 - c_out, Color::BLUEVIOLET);
                            d.draw_circle_v(c_in, 3.0, Color::BLUEVIOLET);
                            d.draw_circle_v(p, 5.0, Color::BLUEVIOLET);
                            d.draw_circle_v(c_out, 3.0, Color::BLUEVIOLET);
                        }
                        None => {
                            if let Some((_, p_last, c_out_last)) = path.points.last() {
                                d.draw_spline_segment_bezier_cubic(
                                    *p_last,
                                    *c_out_last,
                                    c_out,
                                    c_out,
                                    1.0, Color::BLUEVIOLET);
                            }
                            d.draw_circle_v(c_out, 3.0, Color::BLUEVIOLET);
                        }
                    }
                }

                // Artboards foreground
                for board in &document.art_boards {
                    d.draw_text(&board.name, board.rect.x as i32, board.rect.y as i32 - 10, 10, Color::WHITE);
                    d.draw_rectangle_lines_ex(board.rect, 1.0, Color::BLACK);
                }

                // d.draw_circle_v(document.camera.target, 5.0, Color::MAGENTA);
                // d.draw_line_v(document.camera.target, document.camera.target - document.camera.offset, Color::MAGENTA);
            }

            // Draw layers panel
            d.draw_rectangle_rec(layers_panel, Color::new(24,24,24,255));
            const INSET: f32 = 10.0;
            const LAYER_HEIGHT: f32 = 32.0;
            let mut y = INSET;
            for layer in &document.layers {
                d.draw_rectangle_rec(Rectangle::new(layers_panel.x + INSET, layers_panel.y + y, layers_panel.width - INSET * 2.0, LAYER_HEIGHT), Color::GRAY);
                y += LAYER_HEIGHT + INSET;
            }
        }

        mouse_screen_pos_prev = mouse_screen_pos;
    }
}
