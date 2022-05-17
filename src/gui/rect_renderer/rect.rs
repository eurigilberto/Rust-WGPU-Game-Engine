use image::math::Rect;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectGraphic {
    pub rect_position: [f32; 2],   //0
    pub mask_index: u32,           //1
    pub texture_uv: u32,           //2
    pub border_radius: u32,        //3
    pub color_index: u32,          //4
    pub sample_color_channel: u32, //5
    pub border_size: f32,          //6
    pub border_color_index: u32,   //7
    pub element_type: u32,         //8
}

impl Default for RectGraphic {
    fn default() -> Self {
        Self {
            rect_position: Default::default(),
            mask_index: Default::default(),
            texture_uv: Default::default(),
            border_radius: Default::default(),
            color_index: Default::default(),
            sample_color_channel: Default::default(),
            border_size: Default::default(),
            border_color_index: Default::default(),
            element_type: Default::default(),
        }
    }
}

impl RectGraphic {
    pub fn get_vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 9] = wgpu::vertex_attr_array![
            0 => Float32x2,
            1 => Uint32,
            2 => Uint32,
            3 => Uint32,
            4 => Uint32,
            5 => Uint32,
            6 => Float32,
            7 => Uint32,
            8 => Uint32
        ];

        wgpu::VertexBufferLayout {
            step_mode: wgpu::VertexStepMode::Instance,
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            attributes: &ATTRIBUTES,
        }
    }
}
