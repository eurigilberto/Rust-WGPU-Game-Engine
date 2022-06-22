use glam::UVec2;
use winit::{event_loop::EventLoop, window::{WindowBuilder, Window}};

use crate::{render_system, engine_time};

pub struct Engine {
    pub render_system: render_system::RenderSystem,
    pub time: engine_time::EngineTime,

    pub system_bind_group_layout: wgpu::BindGroupLayout,
    pub system_bind_group: wgpu::BindGroup,
}

impl Engine {
    pub fn new(window: &Window) -> Self {
        let render_system = pollster::block_on(render_system::RenderSystem::new(&window));
        let engine_time = engine_time::EngineTime::new(16, &render_system);

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
            time: engine_time,

            system_bind_group_layout,
            system_bind_group,
        }
    }

    pub fn get_screen_size(&self)->UVec2{
        self.render_system.render_window.size
    }
}