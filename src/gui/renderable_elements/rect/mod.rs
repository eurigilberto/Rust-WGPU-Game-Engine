pub mod collection;
pub use collection::RectCollection;

#[repr(C)]
#[derive(Copy, Clone, Debug,  bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectGraphic{
	pub rect_position: [f32;2],
	pub rect_size: [f32;2],
	pub rect_depth: f32,
	pub rect_round: [f32;4],
	pub color: [f32;4],

	pub border_size: f32,
	pub border_color: [f32; 4],
}

impl RectGraphic {
	pub fn get_vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a>{
		const ATTRIBUTES: [wgpu::VertexAttribute; 7] = 
			wgpu::vertex_attr_array![ 
				0 => Float32x2, 
				1 => Float32x2, 
				2 => Float32, 
				3 => Float32x4, 
				4 => Float32x4,
				5 => Float32, 
				6 => Float32x4
			];

		wgpu::VertexBufferLayout{
			step_mode: wgpu::VertexStepMode::Instance,
			array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
			attributes: &ATTRIBUTES
		}
	}
}