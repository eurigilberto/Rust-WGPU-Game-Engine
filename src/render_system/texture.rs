pub fn create_render_texture_descriptor(
    format: wgpu::TextureFormat,
    width: u32,
    height: u32,
    label: Option<&str>,
) -> wgpu::TextureDescriptor {
    wgpu::TextureDescriptor {
        label: label,
        dimension: wgpu::TextureDimension::D2,
        format: format,
        mip_level_count: 1,
        sample_count: 1,
        size: wgpu::Extent3d {
            width: width,
            height: height,
            depth_or_array_layers: 1,
        },
        usage: wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::RENDER_ATTACHMENT,
    }
}

pub fn set_all_address_mode(
    descriptor: &mut wgpu::SamplerDescriptor,
    address_mode: wgpu::AddressMode,
) {
    descriptor.address_mode_u = address_mode;
    descriptor.address_mode_v = address_mode;
    descriptor.address_mode_w = address_mode;
}

pub fn set_all_filters(
	descriptor: &mut wgpu::SamplerDescriptor,
	filter_mode: wgpu::FilterMode
) {
	descriptor.mag_filter = filter_mode;
	descriptor.min_filter = filter_mode;
	descriptor.mipmap_filter = filter_mode;
}

pub fn clear_render_targets(
    encoder: &mut wgpu::CommandEncoder,
    render_texture: &wgpu::TextureView,
    clear_color: wgpu::Color,
    depth_texture: Option<&wgpu::TextureView>,
    depth_clear_value: Option<f32>,
    stencil_clear_value: Option<u32>
) {
    let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Clear Operation"),
        color_attachments: &[wgpu::RenderPassColorAttachment {
            view: &render_texture,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(clear_color),
                store: true,
            },
        }],
        depth_stencil_attachment: match depth_texture {
            Some(texture_view) => {
                Some(wgpu::RenderPassDepthStencilAttachment{
                    view: texture_view,
                    depth_ops: Some(wgpu::Operations{
                        load: wgpu::LoadOp::Clear(depth_clear_value.unwrap_or(0.0)),
                        store: true
                    }),
                    stencil_ops: match stencil_clear_value {
                        Some(clear_value) => {
                            Some(wgpu::Operations{
                                load: wgpu::LoadOp::Clear(clear_value),
                                store: true
                            })
                        },
                        None => {None}
                    }
                })
            },
            None => {None}
        },
    });

    drop(render_pass);
}