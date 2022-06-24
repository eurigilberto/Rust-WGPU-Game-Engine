pub struct OperationTime {
    pub frame_start_time: f64,
    pub event_handling_time: f64,
    pub update_time: f64,
    pub render_time: f64,
	pub frame_end_time: f64
}

impl OperationTime {
    pub fn new() -> Self {
        Self {
            frame_start_time: 0.0,
            event_handling_time: 0.0,
            update_time: 0.0,
            render_time: 0.0,
			frame_end_time: 0.0
        }
    }

	pub fn copy_from(&mut self, other: &Self){
		self.frame_start_time = other.frame_start_time;
		self.event_handling_time = other.event_handling_time;
		self.update_time = other.update_time;
		self.render_time = other.render_time;
		self.frame_end_time = other.frame_end_time;
	}

	pub fn get_total_time(&self) -> f64{
		self.frame_start_time + self. event_handling_time + self.update_time + self.render_time + self.frame_end_time
	}
}
