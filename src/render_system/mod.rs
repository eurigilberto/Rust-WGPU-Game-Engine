mod render_window;
use std::{borrow::Cow, cell::RefCell};

pub mod render_texture;
pub mod texture;
use glam::{uvec2, UVec2};
use render_window::RenderWindow;
use wgpu::{util::DeviceExt, ColorTargetState, VertexBufferLayout};
use winit::event::WindowEvent;

use crate::EngineEvent;
pub mod copy_texture_to_surface;
pub struct RenderSystem {
    pub render_window: RenderWindow,
    
    /// It is a refcell because I don't want to give out mutable references to the entire render system
    /// just because I need a mutable reference to be able to add / consume the destroy texture queue
    destroy_texture_queue: RefCell<Vec<wgpu::Texture>>,
}

pub fn uniform_usage() -> wgpu::BufferUsages {
    wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
}

pub enum TextureSamplerType {
    LinearClampToEdge,
    ClampToEdge
}

impl RenderSystem {
    pub async fn new(window: &winit::window::Window) -> Self {
        let render_window = RenderWindow::new(window).await;
        let destroy_texture_queue = RefCell::new(Vec::<wgpu::Texture>::with_capacity(20));
        Self {
            render_window,
            destroy_texture_queue,
        }
    }
    pub fn resize_event_transformation(event: &EngineEvent) -> Option<UVec2> {
        match event {
            EngineEvent::WinitEvent(WindowEvent::Resized(physical_size)) => {
                let new_size = physical_size.clone();
                return Some(uvec2(new_size.width, new_size.height));
            }
            EngineEvent::ScaleFactorChanged { new_inner_size, scale_factor } => {
                //println!("Required Scale Factor: {:?}", scale_factor);
                return Some(*new_inner_size);
            }
            _ => return None,
        }
    }
    pub fn write_buffer(&self, buffer: &wgpu::Buffer, offset: wgpu::BufferAddress, data: &[u8]) {
        self.render_window.queue.write_buffer(buffer, offset, data);
    }

    pub fn create_buffer_descriptor(
        &self,
        descriptor: &wgpu::util::BufferInitDescriptor,
    ) -> wgpu::Buffer {
        self.render_window.device.create_buffer_init(descriptor)
    }
    pub fn create_buffer(
        &self,
        name: &str,
        data: &[u8],
        usage: wgpu::BufferUsages,
    ) -> wgpu::Buffer {
        let descriptor = wgpu::util::BufferInitDescriptor {
            label: Some(name),
            usage: usage,
            contents: data,
        };
        self.create_buffer_descriptor(&descriptor)
    }

    pub fn configure_surface(&mut self) {
        self.render_window.configure_surface();
    }
    pub fn create_texture(&self, descriptor: &wgpu::TextureDescriptor) -> wgpu::Texture {
        self.render_window.device.create_texture(descriptor)
    }
    pub fn create_sampler(&self, descriptor: &wgpu::SamplerDescriptor) -> wgpu::Sampler {
        self.render_window.device.create_sampler(descriptor)
    }

    pub fn create_shader_module_from_string(
        &self,
        shader_name: &str,
        shader_str: Cow<str>,
    ) -> wgpu::ShaderModule {
        self.render_window
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(shader_name),
                source: wgpu::ShaderSource::Wgsl(shader_str),
            })
    }

    pub fn create_vertex_fragment_state<'a>(
        &self,
        shader_module: &'a wgpu::ShaderModule,
        vertex_entry_point: &'a str,
        vertex_buffer_layouts: &'a [VertexBufferLayout],
        fragment_entry_point: &'a str,
        color_target_states: &'a [Option<ColorTargetState>],
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

    pub fn create_texture_sampler(
        &self,
        label: &str,
        sampler_type: TextureSamplerType,
    ) -> wgpu::Sampler {
        let mut sampler_descriptor = wgpu::SamplerDescriptor {
            label: Some(label),
            ..Default::default()
        };

        match sampler_type {
            TextureSamplerType::LinearClampToEdge => {
                texture::set_all_filters(&mut sampler_descriptor, wgpu::FilterMode::Linear);
                texture::set_all_address_mode(
                    &mut sampler_descriptor,
                    wgpu::AddressMode::ClampToEdge,
                );
                self.render_window
                    .device
                    .create_sampler(&sampler_descriptor)
            }
            TextureSamplerType::ClampToEdge => {
                texture::set_all_address_mode(
                    &mut sampler_descriptor,
                    wgpu::AddressMode::ClampToEdge,
                );
                self.render_window
                    .device
                    .create_sampler(&sampler_descriptor)
            },
            
        }
    }

    pub fn queue_destroy_texture(&self, texture: wgpu::Texture) {
        self.destroy_texture_queue.borrow_mut().push(texture);
    }

    pub fn destroy_queued_textures(&self) {
        for texture in self.destroy_texture_queue.borrow_mut().drain(..) {
            texture.destroy();
        }
    }
}
