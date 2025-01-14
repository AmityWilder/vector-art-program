use raylib::prelude::*;
// use rand::prelude::*;

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

    pub struct Stroke {
        pub blend: Blending,
        pub pattern: Pattern,
        pub thick: f32,
        pub align: Align,
    }
}
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
}
use fill::Fill;

pub enum StyleItem {
    Stroke(Stroke),
    Fill(Fill),
}

pub struct Appearance {
    pub items: Vec<StyleItem>,
}

pub struct VectorPath {
    /// vc cvc ... cvc cv
    pub points: Vec<Vector2>,
    pub appearance: Appearance,
}

impl VectorPath {
    pub fn draw(&self, d: &mut impl RaylibDraw) {
        todo!()
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
    Group(Vec<Layer>),
    Path(VectorPath),
    Rectangle(Rectangle),
    Circle { center: Vector2, radius: f32, },
    Bitmap(Bitmap),
}

pub struct Layer {
    pub name: String,
    pub is_hidden: bool,
    pub blend: Blending,
    pub items: Vec<LayerItem>,
}

impl Layer {
    pub fn new(name: String) -> Self {
        Self {
            name,
            is_hidden: false,
            blend: Blending::default(),
            items: Vec::new(),
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        if !self.is_hidden {
            for item in &self.items {
                match item {
                    LayerItem::Group(group) => for layer in group { layer.draw(d) },
                    LayerItem::Path(path) => path.draw(d),
                    LayerItem::Rectangle(rec) => d.draw_rectangle_rec(rec, Color::WHITE),
                    LayerItem::Circle { center, radius, } => d.draw_circle_v(center, *radius, Color::WHITE),
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
                target: Vector2::new(width * 0.5, height * 0.5),
                rotation: 0.0,
                zoom: 1.0,
            },
            paper_color: Color::GRAY,
            layers: vec![Layer::new("layer 0".to_string())],
            art_boards: vec![ArtBoard::new("artboard 0".to_string(), rrect(0.0, 0.0, width, height))],
        }
    }
}

fn main() {
    let (mut rl, thread) = init()
        .title("Amity Vector Art")
        .resizable()
        .build();

    rl.set_target_fps(60);

    // let mut rng = thread_rng();

    let background_color: Color = Color::new(32,32,32,255);

    let document = Document::new("untitled".to_string(), 256.0, 256.0);

    while !rl.window_should_close() {

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

                // Artboards foreground
                for board in &document.art_boards {
                    d.draw_rectangle_lines_ex(board.rect, 1.0, Color::BLACK);
                }
            }
        }
    }
}
