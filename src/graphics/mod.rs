mod render_surface;
use std::{borrow::Cow, cell::RefCell};

pub mod render_texture;
pub mod texture;
use glam::{uvec2, UVec2};
use render_surface::RenderSurface;
use wgpu::{util::DeviceExt, ColorTargetState, VertexBufferLayout};
use winit::event::WindowEvent;

use crate::EngineEvent;
pub mod copy_texture_to_surface;
pub struct Graphics {
    pub render_window: RenderSurface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    /// It is a refcell because I don't want to give out mutable references to the entire render system
    /// just because I need a mutable reference to be able to add / consume the destroy texture queue
    destroy_texture_queue: RefCell<Vec<wgpu::Texture>>,
}

pub enum TextureSamplerType {
    LinearClampToEdge,
    ClampToEdge,
}

impl Graphics {
    pub async fn new(window: &winit::window::Window) -> Self {
        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Adapter could not be created on Render Window");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .expect("Device and Queue could not be created");

        let size = uvec2(window.inner_size().width, window.inner_size().height);
        let format = surface.get_supported_formats(&adapter)[0];
        let render_window = RenderSurface::new(surface, &device, size, format).await;
        let destroy_texture_queue = RefCell::new(Vec::<wgpu::Texture>::with_capacity(20));

        Self {
            render_window,
            destroy_texture_queue,
            device,
            queue,
        }
    }
    pub fn resize_event_transformation(event: &EngineEvent) -> Option<UVec2> {
        match event {
            EngineEvent::WinitEvent(WindowEvent::Resized(physical_size)) => {
                let new_size = physical_size.clone();
                return Some(uvec2(new_size.width, new_size.height));
            }
            EngineEvent::ScaleFactorChanged {
                new_inner_size,
                scale_factor,
            } => {
                //println!("Required Scale Factor: {:?}", scale_factor);
                return Some(*new_inner_size);
            }
            _ => return None,
        }
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
        self.device.create_buffer_init(&descriptor)
    }

    pub fn create_bind_group(
        &self,
        bind_group_name: Option<&str>,
        layout_descriptor: wgpu::BindGroupLayoutDescriptor,
        bind_group_entries: &[wgpu::BindGroupEntry],
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let layout = self.device.create_bind_group_layout(&layout_descriptor);
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: bind_group_name,
            layout: &layout,
            entries: bind_group_entries,
        });
        (layout, bind_group)
    }

    pub fn configure_surface(&mut self) {
        self.render_window.configure_surface(&self.device);
    }

    pub fn create_shader_module_from_string(
        &self,
        shader_name: &str,
        shader_str: Cow<str>,
    ) -> wgpu::ShaderModule {
        self.device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(shader_name),
                source: wgpu::ShaderSource::Wgsl(shader_str),
            })
    }

    pub fn create_vertex_fragment_state<'a>(
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
        label: Option<&str>,
        sampler_type: TextureSamplerType,
    ) -> wgpu::Sampler {
        let mut sampler_descriptor = wgpu::SamplerDescriptor {
            label: label,
            ..Default::default()
        };

        match sampler_type {
            TextureSamplerType::LinearClampToEdge => {
                texture::set_all_filters(&mut sampler_descriptor, wgpu::FilterMode::Linear);
                texture::set_all_address_mode(
                    &mut sampler_descriptor,
                    wgpu::AddressMode::ClampToEdge,
                );
                self.device.create_sampler(&sampler_descriptor)
            }
            TextureSamplerType::ClampToEdge => {
                texture::set_all_address_mode(
                    &mut sampler_descriptor,
                    wgpu::AddressMode::ClampToEdge,
                );
                self.device.create_sampler(&sampler_descriptor)
            }
        }
    }

    pub fn resize(&mut self, new_size: UVec2){
        self.render_window.resize(&self.device, new_size);
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
