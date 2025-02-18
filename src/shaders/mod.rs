use amymath::prelude::Vector2;
use raylib::prelude::*;

const UV_TEX_SIZE: i32 = 2;
const SRC_REC: Rectangle = {
    const _: () = assert!(UV_TEX_SIZE.ilog2() < f32::MANTISSA_DIGITS); // proof
    const UV_TEX_SIZE_F: f32 =
        #[allow(clippy::cast_precision_loss, reason = "UV_TEX_SIZE can be represented losslessly")]
        (UV_TEX_SIZE as f32);
    Rectangle::new(0.0, 0.0, UV_TEX_SIZE_F, UV_TEX_SIZE_F)
};

pub struct ShaderTable {
    pub circle:    Shader,
    pub bezier:    Shader,
    pub blur:      Shader,
    pub hue_wheel: Shader,
    pub uv_tex: Texture2D,
}

impl ShaderTable {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<Self, String> {
        Ok(Self {
            circle:    rl.load_shader_from_memory(thread, None, Some(include_str!(   "circle.frag"))),
            bezier:    rl.load_shader_from_memory(thread, None, Some(include_str!(   "bezier.frag"))),
            blur:      rl.load_shader_from_memory(thread, None, Some(include_str!(     "blur.frag"))),
            hue_wheel: rl.load_shader_from_memory(thread, None, Some(include_str!("hue_wheel.frag"))),
            uv_tex:    rl.load_texture_from_image(thread, &Image::gen_image_color(UV_TEX_SIZE, UV_TEX_SIZE, Color::WHITE))?,
        })
    }

    pub fn draw_uv_tex_ex(&self, d: &mut impl RaylibDraw, position: Vector2, extent: Vector2, tint: Color) {
        let dest_rec = Rectangle::new(
            position.x - extent.x,
            position.y - extent.y,
            extent.x * 2.0,
            extent.y * 2.0,
        );
        d.draw_texture_pro(&self.uv_tex, SRC_REC, dest_rec, Vector2::ZERO, 0.0, tint);
    }

    #[inline]
    pub fn draw_uv_tex(&self, d: &mut impl RaylibDraw, position: Vector2, radius: f32, tint: Color) {
        self.draw_uv_tex_ex(d, position, Vector2::new(radius, radius), tint);
    }
}
