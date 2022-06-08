#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectGraphic {
    /// Position (x,y) and Size (z,w) - LOC 1
    pub position_size: [u32; 4],
    /// **Data Vector 0** - LOC 1\
    /// 0 - mask_index\
    /// 1 - texture_uv\
    /// 2 - border_radius\
    /// 3 - color_index
    pub data_vector_0: [u32; 4],

    /// **Data Vector 1** - LOC 2\
    /// 0 - border_color_index\
    /// 1 - sample_color_channel\
    /// 2 - border_size\
    /// 3 - element_type
    pub data_vector_1: [u32; 4]
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
            0 => Uint32x4,
            1 => Uint32x4,
            2 => Uint32x4
        ];

        wgpu::VertexBufferLayout {
            step_mode: wgpu::VertexStepMode::Instance,
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            attributes: &ATTRIBUTES,
        }
    }
}
