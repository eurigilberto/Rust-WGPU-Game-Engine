#[derive(Default)]
pub struct OperationTime {
    pub frame_start_time: u128,
    pub event_handling_time: u128,
    pub update_time: u128,
    pub render_time: u128,
	pub frame_end_time: u128,
	pub gpu_lock_time: u128,
}

impl OperationTime {
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

	pub fn get_total_time(&self) -> u128{
		let time =(self.frame_start_time + self.event_handling_time + self.update_time + self.render_time + self.frame_end_time);
		time
	}
}
