use raylib::prelude::*;
use raylib_rs::rlgl::*;
use crate::appearance::Blending;
use super::{LayerSettings, LayerTree, LayerType};

pub struct Group {
    pub settings: LayerSettings,
    pub items: LayerTree,
    pub is_expanded: bool,
    pub blend: Blending,
}

impl Group {
    pub const fn new(settings: LayerSettings, blend: Blending) -> Self {
        Self {
            settings,
            items: LayerTree::new(),
            is_expanded: false,
            blend,
        }
    }
}

impl LayerType for Group {
    fn draw_rendered(&self, d: &mut impl RaylibDraw, camera: &Camera2D, scratch_rtex: &mut [RenderTexture2D]) {
        fn draw_inner(group: &Group, d: &mut impl RaylibDraw, camera: &Camera2D, scratch_rtex: &mut [RenderTexture2D]) {
            for item in group.items.shallow_iter() {
                item.draw_rendered(d, camera, scratch_rtex);
            }
        }

        if self.blend.is_non_default() {
            let ([scratch_rtex], scratch_rtex_rest) = scratch_rtex.split_at_mut(1) else { panic!("insufficient scratch textures") };
            // Safety: the texture mode ends before the method returns, at the same depth it was opened.
            unsafe { ffi::BeginTextureMode(**scratch_rtex); } d.clear_background(Color::BLANK);
            draw_inner(self, d, camera, scratch_rtex_rest);
            unsafe { ffi::EndTextureMode(); }
            let mut d = d.begin_blend_mode(self.blend.mode);
            {
                let mut d = d.rl_set_texture(scratch_rtex.texture.id());
                {
                    let mut d = d.rl_begin_quads();

                    d.rl_normal3f(0.0, 0.0, 1.0);
                    d.rl_color4f(1.0, 1.0, 1.0, self.blend.opacity);

                    d.rl_tex_coord2f(0.0, 0.0); d.rl_vertex2i(0, 0);
                    d.rl_tex_coord2f(0.0, 1.0); d.rl_vertex2i(0, scratch_rtex.height());
                    d.rl_tex_coord2f(1.0, 1.0); d.rl_vertex2i(scratch_rtex.width(), scratch_rtex.height());
                    d.rl_tex_coord2f(1.0, 0.0); d.rl_vertex2i(scratch_rtex.width(), 0);
                }
            }
        } else {
            draw_inner(self, d, camera, scratch_rtex);
        }
    }

    fn draw_selected(&self, d: &mut impl RaylibDraw, px_world_size: f32) {
        for item in self.items.shallow_iter() {
            item.draw_selected(d, px_world_size);
        }
    }
}
