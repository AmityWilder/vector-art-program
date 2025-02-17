pub mod rec;
pub mod panel;

#[cfg(test)]
mod tests {
    use amymath::prelude::*;
    use rltest::*;
    use super::{panel::*, rec::*};

    #[test]
    fn test0() -> rltest::Result {
        rl_test("test0", 1280, 720, 60, |rl| {
            let window_rec = IRect2 {
                min: IVector2::ZERO,
                max: IVector2 {
                    x: rl.get_screen_width (),
                    y: rl.get_screen_height(),
                },
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
