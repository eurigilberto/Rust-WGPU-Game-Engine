use glam::UVec2;
use wgpu::TextureFormat;

pub struct RenderSurface {
    pub surface: wgpu::Surface, //this is problematic, because it forces my thing to only use a single 'window' but the system could easily render to multiple surfaces
    pub config: wgpu::SurfaceConfiguration,
    pub size: UVec2,
    //------------------------------------||
}

impl RenderSurface {
    pub async fn new(
        surface: wgpu::Surface,
        device: &wgpu::Device,
        size: UVec2,
        format: TextureFormat,
    ) -> Self {
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: format,
            width: size.x,
            height: size.y,
            present_mode: wgpu::PresentMode::Immediate,
        };
        surface.configure(device, &config);

        RenderSurface {
            size: size,
            config: config,
            surface: surface,
        }
    }

    //Surface specific things
    pub fn resize(&mut self, device: &wgpu::Device, size: UVec2) {
        if size.x > 2 && size.y > 2 {
            self.size = size;
            self.config.width = self.size.x;
            self.config.height = self.size.y;
            self.configure_surface(device);
        }
    }

    pub fn configure_surface(&mut self, device: &wgpu::Device) {
        self.surface.configure(device, &self.config);
    }
}
