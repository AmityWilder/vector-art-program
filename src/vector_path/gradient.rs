use super::*;

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
        let pos = u8::try_from((t * 255.0).floor() as isize).expect("t must be between 0 and 1");
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
        let t = (t - lower.pos as f32) / (upper.pos as f32 - lower.pos as f32); // normalize
        mix(&lower.color, &upper.color, t)
    }
}
