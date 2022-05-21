use std::borrow::Cow;
use std::num::NonZeroU32;

use glam::uvec2;

use crate::render_system::render_texture::RenderTexture;
use crate::render_system::texture;
use crate::render_system::RenderSystem;

pub fn get_color_target_states() -> [wgpu::ColorTargetState; 2] {
    [
        wgpu::ColorTargetState {
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            format: wgpu::TextureFormat::Rgba16Float,
            write_mask: wgpu::ColorWrites::all(),
        },
        wgpu::ColorTargetState {
            blend: Some(wgpu::BlendState::REPLACE),
            format: wgpu::TextureFormat::R16Uint,
            write_mask: wgpu::ColorWrites::all(),
        },
    ]
}

pub struct GUIRenderTexture<'a> {
    color_texture: RenderTexture<'a>,
    mask_texture: RenderTexture<'a>,
}

impl GUIRenderTexture<'_> {
    pub fn new(render_system: &RenderSystem) -> Self {
        let color_texture = RenderTexture::new(
            wgpu::TextureFormat::Rgba16Float,
            uvec2(1024, 720),
            true,
            render_system,

            "GUI Color Texture",
            "GUI Color Texture View",
            "GUI Color Texture Sampler"
        );

        let mask_texture = RenderTexture::new(
            wgpu::TextureFormat::R16Uint,
            uvec2(1024, 720),
            true,
            render_system,
            "GUI Mask Texture",
            "GUI Mask Texture View",
            "GUI Mask Texture Sampler"
        );

        Self {
            color_texture,
            mask_texture,
        }
    }
}
