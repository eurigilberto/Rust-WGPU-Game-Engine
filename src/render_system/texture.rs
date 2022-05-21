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