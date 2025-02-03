use raylib::prelude::*;

pub struct RaylibRlglLines<'a, D: ?Sized>(&'a mut D);

impl<D: ?Sized> Drop for RaylibRlglLines<'_, D> {
    /// Finish vertex providing
    fn drop(&mut self) {
        unsafe { ffi::rlEnd(); }
    }
}

pub struct RaylibRlglTriangless<'a, D: ?Sized>(&'a mut D);

impl<D: ?Sized> Drop for RaylibRlglTriangless<'_, D> {
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

pub trait RaylibRlglEx {
    /// Initialize drawing mode (how to organize vertex)
    ///
    /// # Safety
    /// idk what to put here
    unsafe fn rl_begin_lines(&mut self) -> RaylibRlglLines<'_, Self> {
        unsafe { ffi::rlBegin(ffi::RL_LINES as i32); }
        RaylibRlglLines(self)
    }

    /// Initialize drawing mode (how to organize vertex)
    ///
    /// # Safety
    /// idk what to put here
    unsafe fn rl_begin_triangles(&mut self) -> RaylibRlglTriangless<'_, Self> {
        unsafe { ffi::rlBegin(ffi::RL_TRIANGLES as i32); }
        RaylibRlglTriangless(self)
    }

    /// Initialize drawing mode (how to organize vertex)
    ///
    /// # Safety
    /// idk what to put here
    unsafe fn rl_begin_quads(&mut self) -> RaylibRlglQuads<'_, Self> {
        unsafe { ffi::rlBegin(ffi::RL_QUADS as i32); }
        RaylibRlglQuads(self)
    }
}

pub struct RlglMatrix<'a, D: ?Sized>(&'a mut D);

impl<D: RaylibDraw> RaylibRlglEx for D {}
impl<D: ?Sized> RaylibRlglEx for RlglMatrix<'_, D> {}

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
impl<D: ?Sized> RaylibRlglDraw for RaylibRlglTriangless<'_, D> {}
impl<D: ?Sized> RaylibRlglDraw for RaylibRlglQuads<'_, D> {}
impl<D: ?Sized> RaylibRlglDraw for RlglMatrix<'_, D> {}
