use amygui::panel::Panel;
use amylib::ops::AnyRange;

pub trait NumericInput {
    type Value;
}

pub struct NumberInput<T> {
    value: T,
    default: T,
    range: AnyRange<T>,
    panel: Panel,
}

impl<T> NumericInput for NumberInput<T> {
    type Value = T;
}

impl<T: Clone> NumberInput<T> {
    pub fn new(default: T, range: AnyRange<T>, panel: Panel) -> Self {
        Self { value: default.clone(), default, range, panel }
    }

    pub fn set_value(&mut self, value: T) where T: PartialOrd {

    }
}

impl NumberInput<i32> {

}

impl NumberInput<usize> {

}

impl NumberInput<f32> {

}