//Create a growable buffer class.
/*This class should have a vector representing the data on memory for fast access on the CPU and a buffer on the GPU
This is meant for sending data contstantly into the GPU, not reading form it. If the buffer in the CPU grows, the
GPU buffer should be disposed of and recreated with the new requiered size. There are limits on the size of the buffer
from the CPU side, and there are better ways to update the buffer than to constantly send it all the data, as there is some
that is not going to change / move position every frame*/

use crate::graphics::Graphics;

pub enum GrowableBufferType {
    VertexBuffer,
    UniformBuffer,
    StorageBuffer,
}

pub struct CPUGPUBuffer<T: bytemuck::Pod> {
    pub cpu_vector: Vec<T>,
    pub gpu_buffer: wgpu::Buffer,
    pub current_capacity: usize,
    pub name: String,
    pub buffer_type: GrowableBufferType,
}

impl<T: bytemuck::Pod> CPUGPUBuffer<T> {
    /// This function only clears the data in the CPU vector
    pub fn clear_buffer(&mut self) {
        self.cpu_vector.clear();
    }

    pub fn push_cpu(&mut self, data: T) -> usize {
        let index = self.cpu_vector.len();
        self.cpu_vector.push(data);
        index
    }
}

pub fn get_buffer_usage_from_buffer_type(buffer_type: &GrowableBufferType) -> wgpu::BufferUsages {
    match buffer_type {
        GrowableBufferType::VertexBuffer => {
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
        }
        GrowableBufferType::UniformBuffer => {
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        }
        GrowableBufferType::StorageBuffer => {
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST
        }
    }
}

pub fn create_cpu_gpu_buffer<T: bytemuck::Pod + Default>(
    buffer_type: GrowableBufferType,
    initial_capacity: usize,
    render_system: &Graphics,
    name: &str,
) -> CPUGPUBuffer<T> {
    let cpu_vector = std::vec::from_elem(T::default(), initial_capacity);
    let gpu_buffer = render_system.create_buffer(
        name,
        bytemuck::cast_slice(cpu_vector.as_slice()),
        get_buffer_usage_from_buffer_type(&buffer_type),
    );
    CPUGPUBuffer::<T> {
        cpu_vector: cpu_vector,
        gpu_buffer: gpu_buffer,
        current_capacity: initial_capacity,
        name: String::from(name),
        buffer_type: buffer_type,
    }
}

pub fn update_buffer<T: bytemuck::Pod>(cpu_gpu_buffer: &mut CPUGPUBuffer<T>, graphics: &Graphics) {
    if cpu_gpu_buffer.current_capacity >= cpu_gpu_buffer.cpu_vector.len() {
        graphics.queue.write_buffer(
            &cpu_gpu_buffer.gpu_buffer,
            0,
            bytemuck::cast_slice(cpu_gpu_buffer.cpu_vector.as_slice()),
        );
    } else {
        cpu_gpu_buffer.gpu_buffer.destroy();
        cpu_gpu_buffer.gpu_buffer = graphics.create_buffer(
            cpu_gpu_buffer.name.as_str(),
            bytemuck::cast_slice(cpu_gpu_buffer.cpu_vector.as_slice()),
            get_buffer_usage_from_buffer_type(&cpu_gpu_buffer.buffer_type),
        );
    }
}

/*pub fn get_binding<T: bytemuck::Pod>(cpu_gpu_buffer: &mut CPUGPUBuffer<T>, graphics: &Graphics) {
    let bind_group_layout =
        graphics
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                }],
            });
    graphics
        .device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: cpu_gpu_buffer.gpu_buffer.as_entire_binding(),
            }],
        });
}*/
