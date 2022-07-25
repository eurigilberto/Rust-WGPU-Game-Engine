use crate::slotmap::{Slotmap, SlotKey};
use crate::graphics::render_texture::RenderTexture;
use crate::graphics::Graphics;
use glam::{uvec2, vec2};

pub fn get_color_target_states() -> [Option<wgpu::ColorTargetState>; 2] {
    [
        Some(wgpu::ColorTargetState {
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            format: wgpu::TextureFormat::Rgba8Unorm,
            write_mask: wgpu::ColorWrites::all(),
        }),
        Some(wgpu::ColorTargetState {
            blend: None,
            format: wgpu::TextureFormat::R8Uint,
            write_mask: wgpu::ColorWrites::all(),
        }),
    ]
}

pub struct GUIRenderTexture {
    pub color_texture_key: SlotKey,
    pub mask_texture_key: SlotKey,
}

impl GUIRenderTexture {
    pub fn new(
        render_system: &Graphics,
        width: u32,
        height: u32,
        render_texture_slotmap: &mut Slotmap<RenderTexture>,
    ) -> Self {
        let color_texture = RenderTexture::create_and_store(
            wgpu::TextureFormat::Rgba8Unorm,
            uvec2(width, height),
            render_system,
            "GUI Color Texture",
            "GUI Color Texture View",
            render_texture_slotmap,
        );

        let mask_texture = RenderTexture::create_and_store(
            wgpu::TextureFormat::R8Uint,
            uvec2(width, height),
            render_system,
            "GUI Mask Texture",
            "GUI Mask Texture View",
            render_texture_slotmap,
        );

        Self {
            color_texture_key: color_texture.expect("Color texture could not be created, check the slotmap to see if there is available space"),
            mask_texture_key: mask_texture.expect("Mask texture could not be created, check the slotmap to see if there is available space"),
        }
    }
}
