use amymath::prelude::*;
use raylib::prelude::*;

pub trait Rect: Sized {
    type Vector: Vector;
    fn minmax_rec(p1: Self::Vector, p2: Self::Vector) -> Self;
    fn entirely_contains(&self, other: &Self) -> bool;
    fn max(self, other: Self) -> Self;
    fn max_pt(self, p: Self::Vector) -> Self;
}

impl Rect for Rect2 {
    type Vector = Vector2;

    fn minmax_rec(p1: Self::Vector, p2: Self::Vector) -> Self {
        Self::minmax_rec(p1, p2)
    }

    fn entirely_contains(&self, other: &Self) -> bool {
        self.entirely_contains(other)
    }

    fn max(self, other: Self) -> Self {
        self.max(other)
    }

    fn max_pt(self, p: Self::Vector) -> Self {
        self.max_pt(p)
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
        self.min.x <= other.min.x && other.max.x <= self.max.x &&
        self.min.y <= other.min.y && other.max.y <= self.max.y &&
        self.min.z <= other.min.z && other.max.z <= self.max.z
    }

    fn max(self, other: Self) -> Self {
        Self {
            min: Vector3 {
                x: self.min.x.min(other.min.x),
                y: self.min.y.min(other.min.y),
                z: self.min.z.min(other.min.z),
            },
            max: Vector3 {
                x: self.max.x.max(other.max.x),
                y: self.max.y.max(other.max.y),
                z: self.max.z.max(other.max.z),
            },
        }
    }

    fn max_pt(self, p: Self::Vector) -> Self {
        Self {
            min: Vector3 {
                x: self.min.x.min(p.x),
                y: self.min.y.min(p.y),
                z: self.min.z.min(p.z),
            },
            max: Vector3 {
                x: self.max.x.max(p.x),
                y: self.max.y.max(p.y),
                z: self.max.z.max(p.z),
            },
        }
    }
}

pub trait Vector where
    Self:
        Copy +
        ReflectVector +
        std::cmp::PartialEq +
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
    type IntoIter: Iterator<Item = f32>;
    const ZERO: Self;
    const ONE: Self;
    const DIM: usize;
    fn into_iter(self) -> Self::IntoIter;
    #[inline]
    fn dot(&self, v: Self) -> f32 {
        (*self * v).into_iter().sum()
    }
    #[inline]
    fn length(&self) -> f32 {
        self.length_sqr().sqrt()
    }
    #[inline]
    fn length_sqr(&self) -> f32 {
        self.dot(*self)
    }
}

impl Vector for Vector2 {
    type Rect = Rect2;
    type IntoIter = <[f32; 2] as IntoIterator>::IntoIter;
    const ZERO: Self = Vector2 { x: 0.0, y: 0.0 };
    const ONE:  Self = Vector2 { x: 1.0, y: 1.0 };
    const DIM: usize = 2;
    fn into_iter(self) -> Self::IntoIter {
        [self.x, self.y].into_iter()
    }
    #[inline]
    fn dot(&self, v: Self) -> f32 {
        self.dot(v)
    }
    #[inline]
    fn length(&self) -> f32 {
        self.length()
    }
    #[inline]
    fn length_sqr(&self) -> f32 {
        self.length_sqr()
    }
}

impl Vector for Vector3 {
    type Rect = BoundingBox;
    type IntoIter = <[f32; 3] as IntoIterator>::IntoIter;
    const ZERO: Self = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    const ONE:  Self = Vector3 { x: 1.0, y: 1.0, z: 1.0 };
    const DIM: usize = 3;
    fn into_iter(self) -> Self::IntoIter {
        [self.x, self.y, self.z].into_iter()
    }
    #[inline]
    fn dot(&self, v: Self) -> f32 {
        self.dot(v)
    }
    #[inline]
    fn length(&self) -> f32 {
        self.length()
    }
    #[inline]
    fn length_sqr(&self) -> f32 {
        self.dot(*self)
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
