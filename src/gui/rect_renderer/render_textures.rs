use glam::uvec2;
use crate::render_system::render_texture::RenderTexture;
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
    pub color_texture: RenderTexture<'a>,
    pub mask_texture: RenderTexture<'a>,
}

impl GUIRenderTexture<'_> {
    pub fn new(render_system: &RenderSystem, width: u32, height: u32) -> Self {
        let color_texture = RenderTexture::new(
            wgpu::TextureFormat::Rgba16Float,
            uvec2(width, height),
            render_system,
            "GUI Color Texture",
            "GUI Color Texture View",
        );

        let mask_texture = RenderTexture::new(
            wgpu::TextureFormat::R16Uint,
            uvec2(width, height),
            render_system,
            "GUI Mask Texture",
            "GUI Mask Texture View",
        );

        Self {
            color_texture,
            mask_texture,
        }
    }
}
