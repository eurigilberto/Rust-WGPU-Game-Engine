use crate::render_system::RenderSystem;
use std::time::Instant;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TimeBufferData {
    pub time: f32,
    pub delta_time: f32,
    pub time_millis: f32,
    pub delta_time_milis: f32,
}
pub struct EngineTime {
    pub time_data: TimeBufferData,
    pub time_buffer: wgpu::Buffer,
    pub last_render_time: Instant,
    
    pub frame_count: u32, 
    accumulated_time: u128,

    pub time_since_start: u128,
    /// How many milliseconds should a frame take
    pub frame_time_micros: u128,
}
impl EngineTime {
    /// Create a timer for the system.
    /// [Frame Time] is measured in miliseconds and represents the requested time between frames.
    /// If the update and render takes longer, then the update for the next frame is started right after the prev one
    pub fn new(frame_time_micros: u128, render_system: &RenderSystem) -> Self {
        let time_data = TimeBufferData {
            time: 0.0,
            delta_time: 0.0,
            time_millis: 0.0,
            delta_time_milis: 0.0,
        };

        let time_buffer = render_system.create_buffer(
            "Engine Time",
            bytemuck::bytes_of(&time_data),
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        );

        Self {
            frame_count: 0,
            accumulated_time: 0,
            time_since_start: 0,
            frame_time_micros,
            time_data: time_data,
            time_buffer: time_buffer,
            last_render_time: std::time::Instant::now(),
        }
    }
    pub fn reset(&mut self) {
        self.frame_count = 0;
        self.accumulated_time = 0;
        self.time_since_start = 0;
        self.last_render_time = std::time::Instant::now();
    }

    /// Returns TRUE if the the system should update
    pub fn update_time(&mut self) -> bool {
        let now = std::time::Instant::now();
        let time_since_last_render = now - self.last_render_time;
        self.accumulated_time = time_since_last_render.as_micros();

        if self.accumulated_time >= self.frame_time_micros {
            //println!("Last Frame Time {}", self.accumulated_time);
            self.frame_count += 1;
            self.last_render_time = now;
            self.time_since_start += self.accumulated_time;

            self.time_data.time_millis = self.time_since_start as f32 / 1000.0;
            self.time_data.delta_time_milis = self.accumulated_time as f32 / 1000.0;

            self.time_data.time = self.time_data.time_millis / 1000.0;
            self.time_data.delta_time = self.time_data.delta_time_milis / 1000.0;

            self.accumulated_time = 0;
            return true;
        }
        return false;
    }

    pub fn update_buffer(&mut self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.time_buffer, 0, bytemuck::bytes_of(&[self.time_data]));
    }
}
