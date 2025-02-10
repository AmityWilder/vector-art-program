use std::ops::{Bound, RangeBounds};
use raylib::prelude::RaylibDrawGui;

pub trait NumericInput {
    type Value;
}

pub struct NumberInput<T> {
    value: T,
    default: T,
    min: Option<T>,
    max: Option<T>,
}

impl<T> NumericInput for NumberInput<T> {
    type Value = T;
}

impl<T: Copy + PartialOrd> NumberInput<T> {
    pub fn new(default: T, range: impl RangeBounds<T>) -> Self {
        assert!(range.contains(&default));
        Self {
            value: default,
            default,
            min: match range.start_bound() {
                Bound::Included(x) | Bound::Excluded(x) => Some(*x),
                Bound::Unbounded => None,
            },
            max: match range.end_bound() {
                Bound::Included(x) | Bound::Excluded(x) => Some(*x),
                Bound::Unbounded => None,
            },
        }
    }

    pub fn tick(&mut self, d: &mut impl RaylibDrawGui) {
        // d.gui_value_box(self., text, value, min_value, max_value, edit_mode)
    }
}

impl NumberInput<i32> {

}

impl NumberInput<usize> {

}

impl NumberInput<f32> {

}
