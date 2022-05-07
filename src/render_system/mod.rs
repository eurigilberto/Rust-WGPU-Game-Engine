mod render_window;
use render_window::RenderWindow;
use winit::event::WindowEvent;
pub struct RenderSystem{
	render_window: RenderWindow
}

impl RenderSystem{
	pub async fn new(window: &winit::window::Window)-> Self{
		let render_window = pollster::block_on(RenderWindow::new(window));
		Self{
			render_window
		}
	}
	pub fn resize_event_handler(&mut self, event: &winit::event::WindowEvent) -> bool{
		match event {
			WindowEvent::Resized(physical_size) => {
				let new_size = physical_size.clone();
				self.render_window.resize(new_size);
				true
			}
			WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
				let new_size = (**new_inner_size).clone();
				self.render_window.resize(new_size);
				true
			}
			_ => {false}
		}
	}
	pub fn render(&mut self) -> Result<(), wgpu::SurfaceError>{
		let output:wgpu::SurfaceTexture = self.render_window.surface.get_current_texture()?;
        let screen_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .render_window.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
		
		//A bunch of render related stuff

		self.render_window.queue.submit(std::iter::once(encoder.finish()));
		output.present();
		Ok(())
	}
	pub fn configure_surface(&mut self){
		self.render_window.configure_surface();
	}
}