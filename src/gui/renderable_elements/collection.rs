use crate::gui::renderable_elements::cpu_gpu_buffer::{
    create_cpu_gpu_buffer, update_buffer, CPUGPUBuffer, GrowableBufferType,
};
use crate::gui::renderable_elements::rect::RectGraphic;
use crate::render_system::RenderSystem;
use crate::{color, render_system};

pub struct RectCollection {
    rect_graphic_cpu_gpu_buffer: CPUGPUBuffer<RectGraphic>,
    rect_mask_cpu_gpu_buffer: CPUGPUBuffer<[f32; 4]>,
    border_radius_cpu_gpu_buffer: CPUGPUBuffer<[f32; 4]>,
    texture_position_cpu_gpu_buffer: CPUGPUBuffer<[u32; 4]>,
    color_cpu_gpu_buffer: CPUGPUBuffer<[f32; 4]>,
}

impl RectCollection {
    pub fn new(initial_capacity: usize, render_system: &RenderSystem) -> Self {
        Self {
            rect_graphic_cpu_gpu_buffer: create_cpu_gpu_buffer::<RectGraphic>(
                GrowableBufferType::VertexBuffer,
                initial_capacity,
                render_system,
                "Rect Collection",
            ),
            rect_mask_cpu_gpu_buffer: create_cpu_gpu_buffer::<[f32; 4]>(
                GrowableBufferType::StorageBuffer,
                initial_capacity,
                render_system,
                "Rect Mask Collection",
            ),
            border_radius_cpu_gpu_buffer: create_cpu_gpu_buffer::<[f32; 4]>(
                GrowableBufferType::StorageBuffer,
                initial_capacity,
                render_system,
                "Border Radius Collection",
            ),
            texture_position_cpu_gpu_buffer: create_cpu_gpu_buffer::<[u32; 4]>(
                GrowableBufferType::StorageBuffer,
                initial_capacity,
                render_system,
                "Texture Position Collection",
            ),
            color_cpu_gpu_buffer: create_cpu_gpu_buffer::<[f32; 4]>(
                GrowableBufferType::StorageBuffer,
                initial_capacity,
                render_system,
                "Color Collection",
            ),
        }
    }

    pub fn update_gpu_buffers(&mut self, render_system: &RenderSystem) {
        update_buffer(&mut self.rect_graphic_cpu_gpu_buffer, render_system);
        update_buffer(&mut self.rect_mask_cpu_gpu_buffer, render_system);
        update_buffer(&mut self.border_radius_cpu_gpu_buffer, render_system);
        update_buffer(&mut self.texture_position_cpu_gpu_buffer, render_system);
        update_buffer(&mut self.color_cpu_gpu_buffer, render_system);
    }
}
