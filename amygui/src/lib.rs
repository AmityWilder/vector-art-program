pub mod rec;
pub mod panel;
pub mod number_input;

#[cfg(test)]
mod tests {
    use amymath::prelude::*;
    use rltest::*;
    use super::{panel::*, rec::*, number_input::*};

    #[test]
    fn test0() -> rltest::Result {
        rl_test("test0", 1280, 720, 60, |rl| {
            let window_rec = IRect2 {
                xmin: 0,
                ymin: 0,
                xmax: rl.get_screen_width (),
                ymax: rl.get_screen_height(),
            };
            let panel = Panel::new(&window_rec, UIRect::init().from_left(0).with_width(50).build(), Color::WHITE);
            rl.run(|rl| {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    success!();
                } else if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
                    failure!("disapproved");
                }
                rl.begin_drawing(Color::BLACK, |d| {
                    let mut d = d.begin_scissor_mode_irect2(panel.rect());
                    d.draw_rectangle_irect2(panel.rect(), Color::BLUE);
                })
            })
        })
    }
}
