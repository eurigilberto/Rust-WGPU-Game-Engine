use crate::render_system::{texture, RenderSystem};
use glam::UVec2;
use std::borrow::Cow;

pub struct RenderTexture<'a> {
    pub texture_descriptor: wgpu::TextureDescriptor<'a>,
    pub texture_view_descriptor: wgpu::TextureViewDescriptor<'a>,

    pub texture: wgpu::Texture,
    pub texture_view: wgpu::TextureView,
    pub texture_sampler: Option<wgpu::Sampler>,
}

impl<'a> RenderTexture<'a> {
    pub fn new(
        format: wgpu::TextureFormat,
        size: UVec2,
        create_sampler: bool,
        render_system: &RenderSystem,

		texture_name: &'a str,
		texture_view_name: &'a str,
		sampler_name: &'a str,
    ) -> Self {

        let texture_descriptor = texture::create_render_texture_descriptor(
            format,
            size.x,
            size.y,
            Some(texture_name));

        let texture_view_descriptor = wgpu::TextureViewDescriptor {
            label: Some(texture_view_name),
            format: Some(format),
            dimension: Some(wgpu::TextureViewDimension::D2),
            aspect: wgpu::TextureAspect::All,
            ..Default::default()
        };

        let texture = render_system.create_texture(&texture_descriptor);

        let texture_view = texture.create_view(&texture_view_descriptor);

        let texture_sampler = if create_sampler {
            let mut sampler_desc = wgpu::SamplerDescriptor {
				label: Some(sampler_name),
				..Default::default()
			};
			texture::set_all_address_mode(&mut sampler_desc, wgpu::AddressMode::ClampToEdge);
			texture::set_all_filters(&mut sampler_desc, wgpu::FilterMode::Linear);
			Some(render_system.create_sampler(&sampler_desc))
        } else {
            None
        };

        Self {
            texture_descriptor,
            texture_view_descriptor,

            texture,
            texture_view,
            texture_sampler,
        }
    }

    pub fn resize_texture(size: UVec2, render_system: &RenderSystem) {

	}
}
