use crate::graphics::Graphics;
use std::num::{NonZeroU32, NonZeroU8};

pub struct TextureAtlas {
    pub texture: wgpu::Texture,
    pub viewer: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl TextureAtlas {
    pub fn new(render_system: &Graphics, width: u32, height: u32, texture_count: u32) -> Self {
        let format = wgpu::TextureFormat::Rgba16Float;

        let texture = render_system.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture Atlas UI"),
            dimension: wgpu::TextureDimension::D2,
            format: format,
            mip_level_count: 1,
            sample_count: 1,
            size: wgpu::Extent3d {
                width: width,
                height: height,
                depth_or_array_layers: texture_count,
            },
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Texture Atlas UI View"),
            format: Some(format),
            aspect: wgpu::TextureAspect::All,
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            base_array_layer: 0,
            base_mip_level: 0,
            mip_level_count: NonZeroU32::new(1),
            array_layer_count: NonZeroU32::new(texture_count),
        });

        let sampler = render_system.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Texture Atlas Sampler"),
            compare: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            anisotropy_clamp: NonZeroU8::new(1),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            border_color: None,
            lod_max_clamp: 1.0,
            lod_min_clamp: -1.0,
            mipmap_filter: wgpu::FilterMode::Linear,
        });

        let (bind_group_layout, bind_group) = render_system.create_bind_group(
            Some("Texture Atlas Bind Group Layout"),
            wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture Atlas Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2Array,
                            multisampled: false,
                        },
                        count: None,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        binding: 0,
                    },
                    wgpu::BindGroupLayoutEntry {
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        binding: 1,
                    },
                ],
            },
            &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        );

        Self {
            texture: texture,
            viewer: texture_view,
            sampler: sampler,
            bind_group_layout,
            bind_group,
        }
    }
}
