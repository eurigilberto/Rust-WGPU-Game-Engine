use crate::gui::rect_renderer::cpu_gpu_buffer::{
    create_cpu_gpu_buffer, update_buffer, CPUGPUBuffer, GrowableBufferType,
};
use crate::gui::rect_renderer::rect::RectGraphic;
use crate::render_system::RenderSystem;
use crate::{color, render_system};

pub struct RectCollection {
    rect_graphic: CPUGPUBuffer<RectGraphic>,
    rect_mask: CPUGPUBuffer<[f32; 4]>,
    border_radius: CPUGPUBuffer<[f32; 4]>,
    texture_position: CPUGPUBuffer<[u32; 4]>,
    color: CPUGPUBuffer<[f32; 4]>,
    storage_bind_group_layout: wgpu::BindGroupLayout,
    storage_bind_group: wgpu::BindGroup
}

fn bind_group_layout_entry(binding_index: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
        binding: binding_index,
        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
    }
}

impl RectCollection {
    pub fn new(initial_capacity: usize, render_system: &RenderSystem) -> Self {
        let rect_graphic = create_cpu_gpu_buffer::<RectGraphic>(
            GrowableBufferType::VertexBuffer,
            initial_capacity,
            render_system,
            "Rect Collection",
        );
        let rect_mask = create_cpu_gpu_buffer::<[f32; 4]>(
            GrowableBufferType::StorageBuffer,
            initial_capacity,
            render_system,
            "Rect Mask Collection",
        );
        let border_radius = create_cpu_gpu_buffer::<[f32; 4]>(
            GrowableBufferType::StorageBuffer,
            initial_capacity,
            render_system,
            "Border Radius Collection",
        );
        let texture_position = create_cpu_gpu_buffer::<[u32; 4]>(
            GrowableBufferType::StorageBuffer,
            initial_capacity,
            render_system,
            "Texture Position Collection",
        );
        let color = create_cpu_gpu_buffer::<[f32; 4]>(
            GrowableBufferType::StorageBuffer,
            initial_capacity,
            render_system,
            "Color Collection",
        );

        let bind_group_layout = render_system.render_window.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Bind Group Layout"),
                entries: &[
                    bind_group_layout_entry(0),
                    bind_group_layout_entry(1),
                    bind_group_layout_entry(2),
                    bind_group_layout_entry(3),
                ],
            },
        );

        let bind_group =
            render_system
                .render_window
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Bind Group"),
                    layout: &bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: rect_mask.gpu_buffer.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: border_radius.gpu_buffer.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: texture_position.gpu_buffer.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 3,
                            resource: color.gpu_buffer.as_entire_binding(),
                        },
                    ],
                });

        Self {
            rect_graphic,
            rect_mask,
            border_radius,
            texture_position,
            color,
            storage_bind_group_layout: bind_group_layout,
            storage_bind_group: bind_group
        }
    }

    pub fn update_gpu_buffers(&mut self, render_system: &RenderSystem) {
        update_buffer(&mut self.rect_graphic, render_system);
        update_buffer(&mut self.rect_mask, render_system);
        update_buffer(&mut self.border_radius, render_system);
        update_buffer(&mut self.texture_position, render_system);
        update_buffer(&mut self.color, render_system);
    }
}
