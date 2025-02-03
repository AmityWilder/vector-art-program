use std::ops::{Bound, RangeBounds};

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

    pub fn set_value(&mut self, value: T) {
        self.value = match (self.min, self.max) {
            (Some(min), _) if value <= min => min,
            (_, Some(max)) if max <= value => max,
            _ => value,
        }
    }

    pub fn reset(&mut self) {
        self.value = self.default;
    }
}

impl NumberInput<i32> {

}

impl NumberInput<usize> {

}

impl NumberInput<f32> {

}
