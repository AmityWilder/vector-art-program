use raylib::prelude::*;

pub trait RaylibRlglExt {
    fn begin_rlgl(&mut self) -> RlglHandle<'_, Self> {
        RlglHandle(self)
    }
}

impl<D: RaylibDraw> RaylibRlglExt for D {}

pub struct RlglHandle<'a, D: ?Sized>(&'a mut D);

impl<D: ?Sized> RlglHandle<'_, D> {
    /// Set current texture to use
    pub unsafe fn rl_set_texture(&mut self, texture: impl AsRef<ffi::Texture2D>) {
        unsafe { ffi::rlSetTexture(texture.as_ref().id); }
    }

    /// Set current texture to use
    pub fn rl_clear_texture(&mut self) {
        unsafe { ffi::rlSetTexture(0); }
    }

    /// Initialize drawing mode (how to organize vertex)
    pub fn rl_begin_lines(&mut self) -> RaylibRlglLines<'_, Self> {
        unsafe { ffi::rlBegin(ffi::RL_LINES as i32); }
        RaylibRlglLines(self)
    }

    /// Initialize drawing mode (how to organize vertex)
    pub fn rl_begin_triangles(&mut self) -> RaylibRlglTriangles<'_, Self> {
        unsafe { ffi::rlBegin(ffi::RL_TRIANGLES as i32); }
        RaylibRlglTriangles(self)
    }

    /// Initialize drawing mode (how to organize vertex)
    pub fn rl_begin_quads(&mut self) -> RaylibRlglQuads<'_, Self> {
        unsafe { ffi::rlBegin(ffi::RL_QUADS as i32); }
        RaylibRlglQuads(self)
    }
}

pub struct RaylibRlglLines<'a, D: ?Sized>(&'a mut D);

impl<D: ?Sized> Drop for RaylibRlglLines<'_, D> {
    /// Finish vertex providing
    fn drop(&mut self) {
        unsafe { ffi::rlEnd(); }
    }
}

pub struct RaylibRlglTriangles<'a, D: ?Sized>(&'a mut D);

impl<D: ?Sized> Drop for RaylibRlglTriangles<'_, D> {
    /// Finish vertex providing
    fn drop(&mut self) {
        unsafe { ffi::rlEnd(); }
    }
}

pub struct RaylibRlglQuads<'a, D: ?Sized>(&'a mut D);

impl<D: ?Sized> Drop for RaylibRlglQuads<'_, D> {
    /// Finish vertex providing
    fn drop(&mut self) {
        unsafe { ffi::rlEnd(); }
    }
}

impl<D: ?Sized> RaylibRlglQuads<'_, D> {
    /// Define one vertex (texture coordinate) - 2 float
    /// NOTE: Texture coordinates are limited to QUADS only
    pub fn rl_tex_coord2f(&mut self, x: f32, y: f32) {
        unsafe { ffi::rlTexCoord2f(x, y); }
    }
}

pub trait RaylibRlglDraw {
    /// Define one vertex (position) - 2 int
    fn rl_vertex2i(&mut self, x: i32, y: i32) {
        unsafe { ffi::rlVertex2i(x, y); }
    }

    /// Define one vertex (position) - 2 float
    fn rl_vertex2f(&mut self, x: f32, y: f32) {
        unsafe { ffi::rlVertex2f(x, y); }
    }

    /// Define one vertex (position) - 3 float
    fn rl_vertex3f(&mut self, x: f32, y: f32, z: f32) {
        unsafe { ffi::rlVertex3f(x, y, z); }
    }

    /// Define one vertex (normal) - 3 float
    /// NOTE: Normals limited to TRIANGLES only?
    fn rl_normal3f(&mut self, x: f32, y: f32, z: f32) {
        unsafe { ffi::rlNormal3f(x, y, z); }
    }

    /// Define one vertex (color) - 4 byte
    fn rl_color4ub(&mut self, r: u8, g: u8, b: u8, a: u8) {
        unsafe { ffi::rlColor4ub(r, g, b, a); }
    }

    /// Define one vertex (color) - 3 float
    fn rl_color3f(&mut self, r: f32, g: f32, b: f32) {
        unsafe { ffi::rlColor3f(r, g, b); }
    }

    /// Define one vertex (color) - 4 float
    fn rl_color4f(&mut self, r: f32, g: f32, b: f32, a: f32) {
        unsafe { ffi::rlColor4f(r, g, b, a); }
    }
}

impl<D: ?Sized> RaylibRlglDraw for RaylibRlglLines<'_, D> {}
impl<D: ?Sized> RaylibRlglDraw for RaylibRlglTriangles<'_, D> {}
impl<D: ?Sized> RaylibRlglDraw for RaylibRlglQuads<'_, D> {}

#[cfg(test)]
mod tests {
    use raylib::prelude::*;
    use rltest::*;
    use super::*;

    #[test]
    fn test0() {
        rl_test("test0", 640, 480, 60, |rl| {
            let texture = rl.load_texture_from_image(&Image::gen_image_gradient_square(32, 32, 0.5, Color::RED, Color::BLUE))?;
            rl.run(|rl| {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    success!()
                }
                rl.begin_drawing(Color::BLACK, |d| {
                    let mut d = d.begin_rlgl();
                    unsafe { d.rl_set_texture(&texture); }
                    {
                        let mut d = d.rl_begin_quads();
                        d.rl_color4ub(255, 255, 255, 255);
                        d.rl_normal3f(0.0, 0.0, 1.0);
                        d.rl_tex_coord2f(0.0, 0.0);
                        d.rl_vertex2f(0.0, 0.0);
                        d.rl_tex_coord2f(0.0, 1.0);
                        d.rl_vertex2f(0.0, 32.0);
                        d.rl_tex_coord2f(1.0, 1.0);
                        d.rl_vertex2f(32.0, 32.0);
                        d.rl_tex_coord2f(1.0, 0.0);
                        d.rl_vertex2f(32.0, 0.0);
                    }
                    d.rl_clear_texture();
                })
            })
        }).expect("rejected");
    }
}
