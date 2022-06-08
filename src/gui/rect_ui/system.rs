use glam::UVec2;

use crate::{render_system::{RenderSystem}, RenderTextureSlotmap};

use super::{
    collection::RectCollection, graphic::RectGraphic, material::RectMaterial,
    render_pass::GUIRenderPassData, render_textures::GUIRenderTexture, texture_atlas::TextureAtlas,
};

pub struct GUIRects {
    pub rect_material: RectMaterial,
    pub render_pass_data: GUIRenderPassData,
    pub rect_collection: RectCollection,
    pub texture_atlas: TextureAtlas,
    pub render_texture: GUIRenderTexture,
}

impl GUIRects {
    pub fn new(
        render_system: &RenderSystem,
        system_bind_group_layout: &wgpu::BindGroupLayout,
        size: UVec2,
        render_texture_slotmap: &mut RenderTextureSlotmap
    ) -> Self {
        let texture_atlas = TextureAtlas::new(render_system, 1024, 1024, 2);
        let rect_collection = RectCollection::new(1024, render_system);
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

        let render_texture = GUIRenderTexture::new(render_system, size.x, size.y, render_texture_slotmap);

        Self {
            rect_material,
            render_pass_data,
            rect_collection,
            texture_atlas,
            render_texture,
        }
    }

    pub fn resize(&mut self, new_size: UVec2, render_system: &RenderSystem){
        self.render_pass_data.resize(new_size, render_system);
    }
}
