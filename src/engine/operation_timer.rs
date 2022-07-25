use super::time::Microsecond;

#[derive(Default)]
pub struct OperationTimer {
    pub frame_start_time: Microsecond,
    pub event_handling_time: Microsecond,
    pub update_time: Microsecond,
    pub render_time: Microsecond,
	pub frame_end_time: Microsecond,
	pub gpu_lock_time: Microsecond,
}

impl OperationTimer {
    pub fn new() -> Self {
        Self::default()
    }

	pub fn copy_from(&mut self, other: &Self){
		self.frame_start_time = other.frame_start_time;
		self.event_handling_time = other.event_handling_time;
		self.update_time = other.update_time;
		self.render_time = other.render_time;
		self.frame_end_time = other.frame_end_time;
		self.gpu_lock_time = other.gpu_lock_time;
	}

	pub fn get_total_time(&self) -> Microsecond{
		let time =(self.frame_start_time + self.event_handling_time + self.update_time + self.render_time + self.frame_end_time);
		time
	}
}
