use glam::{vec2, UVec2};

use crate::{
    render_system::{RenderSystem, render_texture::RenderTexture},
    slotmap::slotmap::Slotmap,
};

use super::{
    collection::RectCollection, graphic::RectGraphic, material::RectMaterial,
    render_pass::GUIRenderPassData, render_textures::GUIRenderTexture, texture_atlas::TextureAtlas,
};

pub struct GUIRects {
    pub rect_material: RectMaterial,
    pub render_pass_data: GUIRenderPassData,
    /// Althoguh it is a public value, it is not meant to be used directly - use element builder instead
    pub rect_collection: RectCollection,
    pub texture_atlas: TextureAtlas,
    pub render_texture: GUIRenderTexture,
}

pub enum ExtraBufferData<T> {
    NewData(T),
    PrevIndex(u16),
}

#[derive(Clone, Copy)]
pub struct BorderRadius {
    pub top_right: f32,
    pub bottom_right: f32,
    pub top_left: f32,
    pub bottom_left: f32,
}

impl Into<[f32;4]> for BorderRadius{
    fn into(self) -> [f32;4] {
        [
            self.top_right,
            self.bottom_right,
            self.top_left,
            self.bottom_left
        ]
    }
}

#[derive(Clone, Copy)]
pub struct RectMask {
    pub position: UVec2,
    pub size: UVec2,
}

impl RectMask {
    pub fn transform_to_gpu(&self, screen_size: UVec2) -> [f32; 4] {
        let start_position = vec2(-1.0, -1.0);
        let bottom_left = start_position
            + (self.position.as_vec2() * 2.0 - self.size.as_vec2()) / screen_size.as_vec2();
        let top_right = start_position
            + (self.position.as_vec2() * 2.0 + self.size.as_vec2()) / screen_size.as_vec2();

        [bottom_left.x, bottom_left.y, top_right.x, top_right.y]
    }
}

impl GUIRects {
    pub fn new(
        render_system: &RenderSystem,
        system_bind_group_layout: &wgpu::BindGroupLayout,
        size: UVec2,
        render_texture_slotmap: &mut Slotmap<RenderTexture>,
    ) -> Self {
        let texture_atlas = TextureAtlas::new(render_system, 1024, 1024, 2);
        let rect_collection = RectCollection::new(126, render_system);
        let render_pass_data = GUIRenderPassData::new(render_system);

        let rect_material = RectMaterial::new(
            render_system,
            &[
                &system_bind_group_layout,
                &render_pass_data.bind_group_layout,
                &rect_collection.uniform_bind_group_layout,
                &texture_atlas.bind_group_layout,
            ],
            &[RectGraphic::get_vertex_buffer_layout()],
        );

        let render_texture =
            GUIRenderTexture::new(render_system, size.x, size.y, render_texture_slotmap);

        Self {
            rect_material,
            render_pass_data,
            rect_collection,
            texture_atlas,
            render_texture,
        }
    }

    pub fn get_color_rt<'a>(&self, rt_slotmap: &'a Slotmap<RenderTexture>)-> &'a RenderTexture{
        rt_slotmap.get_value(&self.render_texture.color_texture_key).expect("GUI Color Render Texture not found")
    }

    pub fn resize(&mut self, new_size: UVec2, render_system: &mut RenderSystem, render_texture_slotmap: &mut Slotmap<RenderTexture>) {
        let color_rt = render_texture_slotmap.get_value_mut(&self.render_texture.color_texture_key).unwrap();
        color_rt.resize_texture(new_size, render_system);

        let mask_rt = render_texture_slotmap.get_value_mut(&self.render_texture.mask_texture_key).unwrap();
        mask_rt.resize_texture(new_size, render_system);

        self.render_pass_data.resize(new_size, render_system);
    }
}