use crate::gui::rect_renderer::render_textures;
use crate::render_system::RenderSystem;

pub struct RectMaterial {
    render_pipeline: wgpu::RenderPipeline,
}

impl RectMaterial {
    pub fn new(
        render_system: &RenderSystem,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        vertex_buffer_layouts: &[wgpu::VertexBufferLayout],
    ) -> Self {
        let color_targets = render_textures::get_color_target_states();

        let gui_quad_shader_str = include_str!("./shader.wgsl");
        let shader_module = render_system.create_shader_module_from_string(
            "GUI Rect Render Shader",
            std::borrow::Cow::Borrowed(gui_quad_shader_str),
        );
        let (vertex_state, fragment_state) = render_system
            .create_vertex_fragment_shader_from_string(
                &shader_module,
                "vs_main",
                vertex_buffer_layouts,
                "fs_main",
                &color_targets,
            );

        let pipeline_layout = render_system.render_window.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("UI Rect Pipeline Layout Descriptor"),
                bind_group_layouts: bind_group_layouts,
                push_constant_ranges: &[],
            },
        );
        let render_pipeline = render_system.render_window.device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("UI Rect Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: vertex_state,
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    ..Default::default()
                },
                fragment: Some(fragment_state),
                multiview: None,
            },
        );

        Self { render_pipeline }
    }
}
