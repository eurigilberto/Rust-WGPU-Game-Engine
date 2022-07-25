use crate::graphics::Graphics;
use std::{
    ops::{Add, AddAssign},
    time::Instant,
};

use super::time::{FrameNumber, Microsecond, Millisecond, Second};

#[repr(C)]
#[derive(Default, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TimeBufferData {
    pub time: Second,
    pub delta_time: Second,
    pub time_millis: Millisecond,
    pub delta_time_milis: Millisecond,
}

pub struct EngineTimer {
    pub time_data: TimeBufferData,
    pub time_buffer: wgpu::Buffer,
    pub last_render_time: Instant,

    pub frame_count: FrameNumber,
    accumulated_time: Microsecond,

    pub time_since_start: Microsecond,
    /// Expected frame duration
    pub frame_duration: Microsecond,
}
impl EngineTimer {
    /// Create a timer for the system.
    /// [Frame Time] is measured in miliseconds and represents the requested time between frames.
    /// If the update and render takes longer, then the update for the next frame is started right after the prev one
    pub fn new(frame_duration: Microsecond, render_system: &Graphics) -> Self {
        let time_data = TimeBufferData::default();

        let time_buffer = render_system.create_buffer(
            "Engine Time",
            bytemuck::bytes_of(&time_data),
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        );

        Self {
            frame_count: FrameNumber(0),
            accumulated_time: Microsecond(0),
            time_since_start: Microsecond(0),
            frame_duration,
            time_data: time_data,
            time_buffer: time_buffer,
            last_render_time: std::time::Instant::now(),
        }
    }
    pub fn reset(&mut self) {
        self.frame_count = FrameNumber(0);
        self.accumulated_time = Microsecond(0);
        self.time_since_start = Microsecond(0);
        self.last_render_time = std::time::Instant::now();
    }

    /// Returns TRUE if the the system should update
    pub fn update_time(&mut self) -> bool {
        let now = std::time::Instant::now();
        let time_since_last_render = now - self.last_render_time;
        self.accumulated_time = Microsecond(time_since_last_render.as_micros());

        if self.accumulated_time >= self.frame_duration {
            //println!("Last Frame Time {}", self.accumulated_time);
            self.frame_count += FrameNumber(1);
            self.last_render_time = now;
            self.time_since_start += self.accumulated_time;

            self.time_data.time_millis = self.time_since_start.as_millisecond();
            self.time_data.delta_time_milis = self.accumulated_time.as_millisecond();

            self.time_data.time = self.time_since_start.as_seconds();
            self.time_data.delta_time = self.accumulated_time.as_seconds();

            self.accumulated_time = Microsecond(0);
            return true;
        }
        return false;
    }

    pub fn update_buffer(&mut self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.time_buffer, 0, bytemuck::bytes_of(&[self.time_data]));
    }
}
