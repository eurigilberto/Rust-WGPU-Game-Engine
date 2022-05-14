//Create a growable buffer class.
/*This class should have a vector representing the data on memory for fast access on the CPU and a buffer on the GPU
This is meant for sending data contstantly into the GPU, not reading form it. If the buffer in the CPU grows, the
GPU buffer should be disposed of and recreated with the new requiered size. There are limits on the size of the buffer
from the CPU side, and there are better ways to update the buffer than to constantly send it all the data, as there is some
that is not going to change / move position every frame*/

use crate::render_system::RenderSystem;

pub enum GrowableBufferType {
    VertexBuffer,
    StorageBuffer,
}

pub struct CPUGPUBuffer<T: bytemuck::Pod> {
    pub cpu_vector: Vec<T>,
    pub gpu_buffer: wgpu::Buffer,
    pub current_capacity: usize,
    pub name: String,
	pub buffer_type: GrowableBufferType
}

pub fn create_cpu_gpu_buffer<T: bytemuck::Pod>(
    buffer_type: GrowableBufferType,
    initial_capacity: usize,
    render_system: &RenderSystem,
    name: &str,
) -> CPUGPUBuffer<T> {
    let cpu_vector = Vec::<T>::with_capacity(initial_capacity);
    let gpu_buffer = match buffer_type {
        GrowableBufferType::VertexBuffer => render_system.create_vertex_buffer(
            name,
            bytemuck::cast_slice(cpu_vector.as_slice()),
            true,
        ),
        GrowableBufferType::StorageBuffer => render_system.create_storage_buffer(
            name,
            bytemuck::cast_slice(cpu_vector.as_slice()),
            true,
        ),
    };
    CPUGPUBuffer::<T> {
        cpu_vector: cpu_vector,
        gpu_buffer: gpu_buffer,
        current_capacity: initial_capacity,
        name: String::from(name),
		buffer_type: buffer_type
    }
}

pub fn update_buffer<T: bytemuck::Pod>(
    cpu_gpu_buffer: &mut CPUGPUBuffer<T>,
    render_system: &RenderSystem
) {
    if cpu_gpu_buffer.current_capacity >= cpu_gpu_buffer.cpu_vector.len() {
        render_system.write_buffer(
            &cpu_gpu_buffer.gpu_buffer,
            0,
            bytemuck::cast_slice(cpu_gpu_buffer.cpu_vector.as_slice()),
        );
    } else {
        cpu_gpu_buffer.gpu_buffer.destroy();
        cpu_gpu_buffer.gpu_buffer = match cpu_gpu_buffer.buffer_type {
            GrowableBufferType::VertexBuffer => render_system.create_vertex_buffer(
                &cpu_gpu_buffer.name,
                bytemuck::cast_slice(cpu_gpu_buffer.cpu_vector.as_slice()),
                true,
            ),
            GrowableBufferType::StorageBuffer => render_system.create_storage_buffer(
                &cpu_gpu_buffer.name,
                bytemuck::cast_slice(cpu_gpu_buffer.cpu_vector.as_slice()),
                true,
            ),
        };
    }
}