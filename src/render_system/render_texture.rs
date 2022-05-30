use crate::{
    render_system::{texture, RenderSystem},
    slotmap::slotmap::Slotmap, EngineSlotmapKeys,
};
use glam::UVec2;
use std::borrow::{Borrow, Cow};

pub struct RenderTexture {
    pub format: wgpu::TextureFormat,
    pub size: UVec2,
    pub texture_name: String,
    pub texture_view_name: String,

    pub texture: wgpu::Texture,
    pub texture_view: wgpu::TextureView,
}

impl RenderTexture {
    pub fn new(
        format: wgpu::TextureFormat,
        size: UVec2,
        render_system: &RenderSystem,

        texture_name: &str,
        texture_view_name: &str,
    ) -> Self {
        let texture_descriptor =
            texture::create_render_texture_descriptor(format, size.x, size.y, Some(texture_name));

        let texture_view_descriptor = wgpu::TextureViewDescriptor {
            label: Some(texture_view_name),
            format: Some(format),
            dimension: Some(wgpu::TextureViewDimension::D2),
            aspect: wgpu::TextureAspect::All,
            ..Default::default()
        };

        let texture = render_system.create_texture(&texture_descriptor);

        let texture_view = texture.create_view(&texture_view_descriptor);

        Self {
            format,
            texture,
            texture_view,
            size,
            texture_name: String::from(texture_name),
            texture_view_name: String::from(texture_view_name),
        }
    }

    pub fn create_and_store(
        format: wgpu::TextureFormat,
        size: UVec2,
        render_system: &RenderSystem,

        texture_name: &str,
        texture_view_name: &str,
        render_texture_slotmap: &mut Slotmap<RenderTexture>,
    ) -> Option<EngineSlotmapKeys> {

        let render_texture = Self::new(format, size, render_system, texture_name, texture_view_name);
        let push_result = render_texture_slotmap.push(render_texture);
        match push_result {
            Some(slot_key) => Some(EngineSlotmapKeys::RenderTexture(slot_key)),
            None => None,
        }
    }

    pub fn get_texture_view_descriptor(&self) -> wgpu::TextureViewDescriptor {
        wgpu::TextureViewDescriptor {
            label: Some(self.texture_name.as_str()),
            format: Some(self.format),
            dimension: Some(wgpu::TextureViewDimension::D2),
            aspect: wgpu::TextureAspect::All,
            ..Default::default()
        }
    }

    pub fn resize_texture(&mut self, new_size: UVec2, render_system: &RenderSystem) {
        let texture_descriptor = texture::create_render_texture_descriptor(
            self.format,
            new_size.x,
            new_size.y,
            Some(self.texture_name.as_str()),
        );
        self.texture.destroy();
        self.texture = render_system.create_texture(&texture_descriptor);

        let texture_view_descriptor = self.get_texture_view_descriptor();
        self.texture_view = self.texture.create_view(&texture_view_descriptor);
    }
}
