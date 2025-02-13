use amymath::prelude::*;
use raylib::prelude::*;

pub trait Rect: Sized {
    type Vector: Vector;
    fn minmax_rec(p1: Self::Vector, p2: Self::Vector) -> Self;
    fn entirely_contains(&self, other: &Self) -> bool;
    fn max(&self, other: &Self) -> Self;
}

impl Rect for Rect2 {
    type Vector = Vector2;

    fn minmax_rec(p1: Self::Vector, p2: Self::Vector) -> Self {
        Self::minmax_rec(p1, p2)
    }

    fn entirely_contains(&self, other: &Self) -> bool {
        todo!()
    }

    fn max(&self, other: &Self) -> Self {
        todo!()
    }
}

impl Rect for BoundingBox {
    type Vector = Vector3;

    fn minmax_rec(p1: Self::Vector, p2: Self::Vector) -> Self {
        Self {
            min: Vector3 {
                x: p1.x.min(p2.x),
                y: p1.y.min(p2.y),
                z: p1.z.min(p2.z),
            },
            max: Vector3 {
                x: p1.x.max(p2.x),
                y: p1.y.max(p2.y),
                z: p1.z.max(p2.z),
            },
        }
    }

    fn entirely_contains(&self, other: &Self) -> bool {
        todo!()
    }

    fn max(&self, other: &Self) -> Self {
        todo!()
    }
}

pub trait Vector where
    Self:
        Copy +
        ReflectVector +
        std::ops::Neg<Output = Self> +
        std::ops::Add<Self, Output = Self> +
        std::ops::Sub<Self, Output = Self> +
        std::ops::Mul<Self, Output = Self> +
        std::ops::Div<Self, Output = Self> +
        std::ops::Mul<f32, Output = Self> +
        std::ops::Div<f32, Output = Self> +
        std::ops::AddAssign<Self> +
        std::ops::SubAssign<Self> +
        std::ops::MulAssign<Self> +
        std::ops::DivAssign<Self> +
        std::ops::MulAssign<f32> +
        std::ops::DivAssign<f32>,
{
    type Rect: Rect<Vector = Self>;
    const ZERO: Self;
    const ONE: Self;
    const DIM: usize;
    fn into_arr(self) -> [f32; Self::DIM];
}

impl Vector for Vector2 {
    type Rect = Rect2;
    const ZERO: Self = Vector2 { x: 0.0, y: 0.0 };
    const ONE:  Self = Vector2 { x: 1.0, y: 1.0 };
    const DIM: usize = 2;
    fn into_arr(self) -> [f32; Self::DIM] {
        [self.x, self.y]
    }
}

impl Vector for Vector3 {
    type Rect = BoundingBox;
    const ZERO: Self = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    const ONE:  Self = Vector3 { x: 1.0, y: 1.0, z: 1.0 };
    const DIM: usize = 3;
    fn into_arr(self) -> [f32; Self::DIM] {
        [self.x, self.y, self.z]
    }
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
