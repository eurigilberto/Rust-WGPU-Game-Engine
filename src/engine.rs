use winit::{event_loop::EventLoop, window::WindowBuilder};

use crate::{render_system, engine_time};

pub struct Engine {
    pub render_system: render_system::RenderSystem,
    pub window: winit::window::Window,
    pub time: engine_time::EngineTime,

    pub system_bind_group_layout: wgpu::BindGroupLayout,
    pub system_bind_group: wgpu::BindGroup,
}

impl Engine {
	fn create_default_window(
		event_loop: &EventLoop<()>,
		width: u32,
		height: u32,
		title: &str,
	) -> winit::window::Window {
		let window = WindowBuilder::new()
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
			.with_decorations(true)
			.build(event_loop)
			.expect("Window could not be created");
		window.set_title(title);
		window
	}

    pub fn new(width: u32, height: u32, title: &str, event_loop: &EventLoop<()>) -> Self {
        let window = Engine::create_default_window(event_loop, width, height, title);
        let render_system = pollster::block_on(render_system::RenderSystem::new(&window));
        let engine_time = engine_time::EngineTime::new(11, &render_system);

        let system_bind_group_layout = render_system.render_window.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("System Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            },
        );
        let system_bind_group =
            render_system
                .render_window
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("System Bind Group"),
                    layout: &system_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: engine_time.time_buffer.as_entire_binding(),
                    }],
                });

        Self {
            render_system,
            window,
            time: engine_time,

            system_bind_group_layout,
            system_bind_group,
        }
    }
}