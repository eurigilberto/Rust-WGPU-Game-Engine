use glam::{UVec2, uvec2};

pub struct RenderWindow {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: UVec2,
}

impl RenderWindow {
    pub async fn new(window: &winit::window::Window) -> Self {
        let size = uvec2(window.inner_size().width, window.inner_size().height);

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

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).expect("Could not get prefered format"),
            width: size.x,
            height: size.y,
            present_mode: wgpu::PresentMode::Immediate,
        };
        surface.configure(&device, &config);

        RenderWindow {
            size: size,
            config: config,
            device: device,
            queue: queue,
            surface: surface,
        }
    }

    pub fn resize(&mut self, size: UVec2) {
        if size.x > 2 && size.y > 2 {
            self.size = size;
            self.config.width = self.size.x;
            self.config.height = self.size.y;
            self.configure_surface();
        }
    }

    pub fn configure_surface(&mut self){
        self.surface.configure(&self.device, &self.config);
    }
}
