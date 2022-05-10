pub mod texture_utils {
    pub fn clear_render_targets(
        encoder: &mut wgpu::CommandEncoder,
        render_texture: &wgpu::TextureView,
        clear_color: wgpu::Color,
        depth_texture: Option<&wgpu::TextureView>,
        depth_clear_value: f32,
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
                            load: wgpu::LoadOp::Clear(depth_clear_value),
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
}
