use std::ops::{Deref, DerefMut};

pub use glam;
pub mod color;
mod engine_time;
pub mod font;
pub mod gui;
pub mod render_system;

pub use bytemuck;
pub use wgpu;
pub use winit;
pub mod slotmap;

pub mod entity_component;
pub use self::entity_component::{
    EngineDataSlotmapTypes, EngineSlotmapKeys, EngineSystemTypes, RenderTextureSlotmap,
};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

fn create_window(
    event_loop: &EventLoop<()>,
    width: u32,
    height: u32,
    title: &str,
) -> winit::window::Window {
    let window = WindowBuilder::new()
        .with_decorations(true)
        .build(event_loop)
        .unwrap();
    window.set_title(title);
    window.set_inner_size(winit::dpi::LogicalSize::new(width, height));
    /*window
    .set_cursor_grab(true)
    .expect("Windows does not support cursor Grab");*/
    //window.set_cursor_visible(false);
    //window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
    window
}

fn close_event_request_handler(
    event: &winit::event::WindowEvent,
    control_flow: &mut ControlFlow,
) -> bool {
    match event {
        WindowEvent::CloseRequested
        | WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
            ..
        } => {
            *control_flow = ControlFlow::Exit;
            true
        }
        _ => false,
    }
}

pub struct Engine {
    pub render_system: render_system::RenderSystem,
    pub window: winit::window::Window,
    pub time: engine_time::EngineTime,

    pub system_bind_group_layout: wgpu::BindGroupLayout,
    pub system_bind_group: wgpu::BindGroup,
}

pub trait Runtime {
    fn handle_event_queue(&mut self, event_queue: &Vec<winit::event::WindowEvent>);
    fn update(&mut self);
    fn render(
        &mut self,
        engine: &mut Engine,
        screen_view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
    ) -> bool;
    fn frame_end(&mut self);
}

pub fn render<R: 'static + Runtime>(
    engine: &mut Engine,
    runtime: &mut R,
) -> Result<(), wgpu::SurfaceError> {
    let output: wgpu::SurfaceTexture = engine
        .render_system
        .render_window
        .surface
        .get_current_texture()?;
    let screen_view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = engine
        .render_system
        .render_window
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

    runtime.render(engine, &screen_view, &mut encoder);

    let mut command_buffers = Vec::<wgpu::CommandBuffer>::new();
    command_buffers.push(encoder.finish());
    engine
        .render_system
        .render_window
        .queue
        .submit(command_buffers);
    output.present();

    Ok(())
}

pub fn start_engine_loop<R: 'static + Runtime>(mut engine: Engine, mut runtime: R, event_loop: EventLoop<()>) {
    engine.time.reset();
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == engine.window.id() => {
                if !close_event_request_handler(event, control_flow)
                    && !engine.render_system.resize_event_handler(event)
                {
                    //Window events -- run through the list of input maps
                }
            }
            Event::MainEventsCleared => {
                if engine.time.update_time() {
                    engine
                        .time
                        .update_buffer(&engine.render_system.render_window.queue);

                    let events = Vec::<winit::event::WindowEvent>::new();
                    runtime.handle_event_queue(&events);
                    runtime.update();
                    let render_result = render(&mut engine, &mut runtime);
                    match render_result {
                        Ok(_) => {
                            runtime.frame_end();
                        }
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => engine.render_system.configure_surface(),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            *control_flow = ControlFlow::Exit
                        }
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
            }
            _ => {}
        }
    });
}

impl Engine {
    fn new(width: u32, height: u32, title: &str, event_loop: &EventLoop<()>) -> Self {
        
        let window = create_window(event_loop, width, height, title);
        let render_system = pollster::block_on(render_system::RenderSystem::new(&window));
        let engine_time = engine_time::EngineTime::new(11, &render_system);

        let system_bind_group_layout = render_system.render_window.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("System Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            },
        );
        let system_bind_group =
            render_system
                .render_window
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("System Bind Group"),
                    layout: &system_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: engine_time.time_buffer.as_entire_binding(),
                    }],
                });

        Self {
            render_system,
            window,
            time: engine_time,

            system_bind_group_layout,
            system_bind_group,
        }
    }
}

pub fn create_engine(width: u32, height: u32, title: &str) -> (Engine, EventLoop<()>){
    let event_loop = EventLoop::new();
    let engine = Engine::new(width, height, title, &event_loop);

    (engine, event_loop)
}