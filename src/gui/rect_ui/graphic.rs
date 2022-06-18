#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectGraphic {
    /// Position (x,y) and Size (z,w) - LOC 1
    pub position_size: [f32; 4],
    /// **Data Vector 0** - LOC 1 See gui structure md for more info
    pub data_vector_0: [u32; 4],

    /// **Data Vector 1** - LOC2 See gui structure md for more info
    pub data_vector_1: [f32; 4]
}

impl Default for RectGraphic {
    fn default() -> Self {
        Self {
            position_size: Default::default(),
            data_vector_0: Default::default(),
            data_vector_1: Default::default()
        }
    }
}

impl RectGraphic {
    pub fn get_vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
            0 => Float32x4,
            1 => Uint32x4,
            2 => Float32x4
        ];

        wgpu::VertexBufferLayout {
            step_mode: wgpu::VertexStepMode::Instance,
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            attributes: &ATTRIBUTES,
        }
    }
}
