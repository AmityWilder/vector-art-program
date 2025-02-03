use amymath::prelude::*;
use raylib::prelude::*;

pub struct Control {
    pub pos: u8,
    pub color: Color,
}

pub struct Ramp {
    /// Should be sorted and unique by `pos`
    pub colors: Vec<Control>,
}

impl Ramp {
    pub fn clean(&mut self) {
        self.colors.sort_by_key(|x| x.pos);
        self.colors.dedup_by_key(|x| x.pos);
    }

    pub fn color_at(&self, t: f32) -> Color {
        let pos =
            #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation, reason = "value is both clamped and rounded")]
            ((t.clamp(0.0, 1.0) * 255.0).round() as isize as u8);
        let mut lower: &Control = &self.colors[0];
        let mut upper: &Control = &self.colors[0];
        // todo: use binary search
        for ctrl in &self.colors {
            if ctrl.pos <= pos && ctrl.pos > lower.pos {
                lower = ctrl;
            }
            if ctrl.pos > pos && ctrl.pos < upper.pos {
                upper = ctrl;
            }
        }
        let t = (t - f32::from(lower.pos)) / (f32::from(upper.pos) - f32::from(lower.pos)); // normalize
        lower.color.mix(upper.color, t)
    }
}
