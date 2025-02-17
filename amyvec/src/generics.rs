use amymath::prelude::*;
use raylib::prelude::*;

pub trait Rect: Sized + Union + Union<Self::Vector> + Overlap {
    type Vector: Vector;
}

pub trait Maternal<V: Vector>: Copy {
    fn transform(&self, v: V) -> V;
}

impl Maternal<Vector2> for Matrix2x2 {
    fn transform(&self, v: Vector2) -> Vector2 {
        self * v
    }
}
impl Maternal<Vector3> for Matrix {
    fn transform(&self, v: Vector3) -> Vector3 {
        v.transform_with(*self)
    }
}
