mod render_window;
mod utils;
use render_window::RenderWindow;
use wgpu::util::DeviceExt;
use winit::event::WindowEvent;
pub struct RenderSystem {
    pub render_window: RenderWindow,
}

impl RenderSystem {
    pub async fn new(window: &winit::window::Window) -> Self {
        let render_window = RenderWindow::new(window).await;
        Self { render_window }
    }
    pub fn resize_event_handler(&mut self, event: &winit::event::WindowEvent) -> bool {
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
            _ => false,
        }
    }
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output: wgpu::SurfaceTexture = self.render_window.surface.get_current_texture()?;
        let screen_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            self.render_window
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
		
        utils::texture_utils::clear_render_targets(
            &mut encoder,
            &screen_view,
            wgpu::Color {
                r: 1.0,
                g: 0.5,
                b: 0.0,
                a: 1.0,
            },
            None,
            0.0,
            None,
        );
        //A bunch of render related stuff

        self.render_window
            .queue
            .submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
    pub fn create_buffer(&self, descriptor: &wgpu::util::BufferInitDescriptor) -> wgpu::Buffer {
        self.render_window.device.create_buffer_init(descriptor)
    }
    pub fn create_uniform_buffer(&self, name: &str, data: &[u8], allow_update: bool) -> wgpu::Buffer {
		let mut usage = wgpu::BufferUsages::UNIFORM;
		if allow_update {
			usage |= wgpu::BufferUsages::COPY_DST;
		}
        self.create_buffer(&wgpu::util::BufferInitDescriptor {
			label: Some(name),
			usage:  usage,
			contents: data,
		})
    }
	pub fn create_vertex_buffer(&self, name: &str, data: &[u8], allow_update: bool) -> wgpu::Buffer{
		let mut usage = wgpu::BufferUsages::VERTEX;
		if allow_update {
			usage |= wgpu::BufferUsages::COPY_DST;
		}
		self.create_buffer(&wgpu::util::BufferInitDescriptor {
			label: Some(name),
			usage:  usage,
			contents: data,
		})
	}
    pub fn configure_surface(&mut self) {
        self.render_window.configure_surface();
    }
}
