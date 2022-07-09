use super::RenderSystem;

pub struct CopyTextureToSurface {
    pub render_pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    texture_sampler: wgpu::Sampler,
}

impl CopyTextureToSurface {
    pub fn new(render_system: &RenderSystem, texture_view: &wgpu::TextureView) -> Self {
        let copy_texture_shader = render_system
            .render_window
            .device
            .create_shader_module(wgpu::include_wgsl!("copy_texture_to_surface_shader.wgsl"));

        let bind_group_layout = render_system.render_window.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Copy Texture To Surface Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        count: None,
                        binding: 0,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                    },
                    wgpu::BindGroupLayoutEntry {
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        count: None,
                        binding: 1,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    },
                ],
            },
        );

        let pipeline_layout = render_system.render_window.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Copy Texture To Surface Pipeline Layout"),
                push_constant_ranges: &[],
                bind_group_layouts: &[&bind_group_layout],
            },
        );

        let render_pipeline = render_system.render_window.device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Copy Texture To Surface Pipeline"),
                layout: Some(&pipeline_layout),
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                vertex: wgpu::VertexState {
                    entry_point: "vs_main",
                    module: &copy_texture_shader,
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    entry_point: "fs_main",
                    module: &copy_texture_shader,
                    targets: &[Some(wgpu::ColorTargetState {
                        write_mask: wgpu::ColorWrites::ALL,
                        format: render_system.render_window.config.format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    })],
                }),
            },
        );

        let texture_sampler = render_system.create_texture_sampler(
            "Copy Texture to Surface Sampler",
            super::TextureSamplerType::ClampToEdge,
        );

        let bind_group = CopyTextureToSurface::create_bind_group(
            &bind_group_layout,
            &render_system.render_window.device,
            texture_view,
            &texture_sampler,
        );

        Self {
            render_pipeline,
            bind_group_layout,
            bind_group,
            texture_sampler,
        }
    }

    pub fn create_bind_group(
        bind_group_layout: &wgpu::BindGroupLayout,
        device: &wgpu::Device,
        texture_view: &wgpu::TextureView,
        texture_sampler: &wgpu::Sampler,
    ) -> wgpu::BindGroup {
        //This function should be used to create the bind group and buffers necesary to be used on the copy texture to screen
        //The render textures are rarely going to change or be updated suddenly so an object that lasts as long as the texture
        //is alive should be created

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Copy Texture to Surface Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(texture_sampler),
                },
            ],
        })
    }

    pub fn update_texture_view(
        &mut self,
        texture_view: &wgpu::TextureView,
        render_system: &RenderSystem,
    ) {
        self.bind_group = Self::create_bind_group(
            &self.bind_group_layout,
            &render_system.render_window.device,
            texture_view,
            &self.texture_sampler,
        );
    }

    pub fn render(&mut self, encoder: &mut wgpu::CommandEncoder, screen_view: &wgpu::TextureView) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Copy Texture to Surface Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &screen_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..4, 0..1);

        drop(render_pass);
    }
}
