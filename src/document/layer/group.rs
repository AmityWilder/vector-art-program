use amymath::rlgl::*;
use raylib::prelude::*;
use super::{LayerSettings, LayerTree, LayerType};

pub struct Group {
    pub settings: LayerSettings,
    pub items: LayerTree,
    pub is_expanded: bool,
}

impl Group {
    pub const fn new(settings: LayerSettings) -> Self {
        Self {
            settings,
            items: LayerTree::new(),
            is_expanded: false,
        }
    }
}

impl LayerType for Group {
    fn draw_rendered(&self, d: &mut impl RaylibDraw, scratch_rtex: &mut [RenderTexture2D]) {
        let ([scratch_rtex], scratch_rtex_rest) = scratch_rtex.split_at_mut(1) else { panic!("insufficient scratch textures") };
        // Safety: the texture mode ends before the method returns, at the same scope it was opened.
        unsafe { ffi::BeginTextureMode(**scratch_rtex); }
        for item in self.items.shallow_iter() {
            item.draw_rendered(d, scratch_rtex_rest);
        }
        unsafe { ffi::EndTextureMode(); }
        let mut d = d.rl_set_texture(scratch_rtex.texture.id);
        {
            let mut d = d.rl_begin_quads();

            d.rl_normal3f(0.0, 0.0, 1.0);
            d.rl_color4f(1.0, 1.0, 1.0, self.settings.blend.opacity);

            d.rl_tex_coord2f(0.0, 0.0);
            d.rl_vertex2i(0, scratch_rtex.height());

            d.rl_tex_coord2f(0.0, 1.0);
            d.rl_vertex2i(0, 0);

            d.rl_tex_coord2f(1.0, 1.0);
            d.rl_vertex2i(scratch_rtex.width(), 0);

            d.rl_tex_coord2f(1.0, 0.0);
            d.rl_vertex2i(scratch_rtex.width(), scratch_rtex.height());
        }
    }

    fn draw_selected(&self, d: &mut impl RaylibDraw, px_world_size: f32) {
        for item in self.items.shallow_iter() {
            item.draw_selected(d, px_world_size);
        }
    }
}
