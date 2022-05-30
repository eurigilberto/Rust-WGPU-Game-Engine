use glam::uvec2;
use crate::{EngineSlotmapKeys, RenderTextureSlotmap};
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

pub struct GUIRenderTexture {
    pub color_texture_key: EngineSlotmapKeys,
    pub mask_texture_key: EngineSlotmapKeys,
}

impl GUIRenderTexture {
    pub fn new(render_system: &RenderSystem, width: u32, height: u32, render_texture_slotmap: &mut RenderTextureSlotmap) -> Self {
        let color_texture = RenderTexture::create_and_store(
            wgpu::TextureFormat::Rgba16Float,
            uvec2(width, height),
            render_system,
            "GUI Color Texture",
            "GUI Color Texture View",
            render_texture_slotmap
        );

        let mask_texture = RenderTexture::create_and_store(
            wgpu::TextureFormat::R16Uint,
            uvec2(width, height),
            render_system,
            "GUI Mask Texture",
            "GUI Mask Texture View",
            render_texture_slotmap
        );

        if let None = color_texture {
            panic!("Color texture could not be created, check the slotmap to see if there is available space")
        }

        if let None = mask_texture {
            panic!("Mask texture could not be created, check the slotmap to see if there is available space")
        }

        Self {
            color_texture_key: color_texture.unwrap(),
            mask_texture_key: mask_texture.unwrap(),
        }
    }
}
