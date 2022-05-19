pub fn get_color_target_states() -> [wgpu::ColorTargetState;2] {
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
