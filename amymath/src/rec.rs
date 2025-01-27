use raylib::prelude::*;

pub trait MinMaxRectangle {
    fn minmax_rec(&self, other: Self) -> Rectangle;
}

impl MinMaxRectangle for Vector2 {
    fn minmax_rec(&self, other: Self) -> Rectangle {
        Rectangle {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            width:  (other.x - self.x).abs(),
            height: (other.y - self.y).abs(),
        }
    }
}

