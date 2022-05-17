mod render_window;
mod utils;
use std::borrow::Cow;

use render_window::RenderWindow;
use wgpu::{util::DeviceExt, VertexBufferLayout, ColorTargetState};
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
    pub fn write_buffer(&self, buffer: &wgpu::Buffer, offset: wgpu::BufferAddress, data: &[u8]){
        self.render_window.queue.write_buffer(buffer, offset, data);
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
    pub fn create_storage_buffer(&self, name: &str, data: &[u8], allow_update:bool) -> wgpu::Buffer{
        let mut usage = wgpu::BufferUsages::STORAGE;
        if allow_update {
            usage |= wgpu::BufferUsages::COPY_DST;
        }
        self.create_buffer(&wgpu::util::BufferInitDescriptor{
            label: Some(name),
            usage: usage,
            contents:data
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
    pub fn create_texture(&self, descriptor: &wgpu::TextureDescriptor) -> wgpu::Texture{
        self.render_window.device.create_texture(descriptor)
    }
    pub fn create_sampler(&self, descriptor: &wgpu::SamplerDescriptor) -> wgpu::Sampler{
        self.render_window.device.create_sampler(descriptor)
    }

    pub fn create_shader_module_from_string(&self, shader_name: &str, shader_str: Cow<str>)->wgpu::ShaderModule{
        self
            .render_window
            .device
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some(shader_name),
                source: wgpu::ShaderSource::Wgsl(shader_str),
            })
    }
    
    pub fn create_vertex_fragment_shader_from_string<'a>(
        &self,
        shader_module: &'a wgpu::ShaderModule,
        vertex_entry_point: &'a str,
        vertex_buffer_layouts: &'a [VertexBufferLayout],
        fragment_entry_point: &'a str,
        color_target_states: &'a [ColorTargetState],
    ) -> (wgpu::VertexState<'a>, wgpu::FragmentState<'a>) {
        let vertex_state = wgpu::VertexState {
            module: shader_module,
            entry_point: vertex_entry_point,
            buffers: vertex_buffer_layouts,
        };
    
        let fragment_state = wgpu::FragmentState {
            module: shader_module,
            entry_point: fragment_entry_point,
            targets: color_target_states,
        };
    
        (vertex_state, fragment_state)
    }
}
