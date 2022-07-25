use glam::UVec2;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub mod time;
use time::*;
pub mod engine_timer;
pub mod operation_timer;
use crate::graphics;
use operation_timer::*;

pub struct Engine {
    pub graphics: graphics::Graphics,
    pub timer: engine_timer::EngineTimer,
    pub operation_timer: operation_timer::OperationTimer,

    pub system_bind_group_layout: wgpu::BindGroupLayout,
    pub system_bind_group: wgpu::BindGroup,
}

impl Engine {
    pub fn new(window: &Window, frame_time_micros: Microsecond) -> Self {
        let render_system = pollster::block_on(graphics::Graphics::new(&window));
        let engine_time = engine_timer::EngineTimer::new(frame_time_micros, &render_system);

        let (system_bind_group_layout, system_bind_group) = render_system.create_bind_group(
            Some("System Bind Group"),
            wgpu::BindGroupLayoutDescriptor {
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
            &[wgpu::BindGroupEntry {
                binding: 0,
                resource: engine_time.time_buffer.as_entire_binding(),
            }]
        );

        Self {
            graphics: render_system,
            timer: engine_time,
            operation_timer: OperationTimer::new(),
            system_bind_group_layout,
            system_bind_group,
        }
    }

    pub fn get_screen_size(&self) -> UVec2 {
        self.graphics.render_window.size
    }
}
