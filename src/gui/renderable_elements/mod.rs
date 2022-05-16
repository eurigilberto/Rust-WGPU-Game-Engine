use std::num::NonZeroU32;

use crate::render_system::RenderSystem;
use wgpu::util::DeviceExt;

pub mod cpu_gpu_buffer;
pub mod collection;
pub mod rect;
pub mod texture_atlas;

/*pub fn test_create_texture_array(render_system: &RenderSystem) {
	let texture_size = wgpu::Extent3d {
		width: 1024,
		height: 1024,
		depth_or_array_layers: 10,
	};
    let texture = render_system
        .render_window
        .device
        .create_texture(&wgpu::TextureDescriptor {
			label: Some("Test Texture"),
			size: texture_size,
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: wgpu::TextureFormat::Rgba16Float,
			usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });
	render_system
		.render_window
		.queue
		.write_texture(
			texture.as_image_copy(), 
			&[], 
			wgpu::ImageDataLayout{
				offset: 0,
				bytes_per_row: NonZeroU32::new(texture_size.width * 8),
				rows_per_image: None
			},
			texture_size
		);
}*/