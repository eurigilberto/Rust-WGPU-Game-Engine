pub use glam;
pub mod color;
mod engine_time;
pub mod font;
pub mod gui;
pub mod render_system;
pub use wgpu;
pub use bytemuck;
pub mod slotmap;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
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
    pub engine_time: engine_time::EngineTime,
    pub event_loop: EventLoop<()>,

    pub system_bind_group_layout: wgpu::BindGroupLayout,
    pub system_bind_group: wgpu::BindGroup,
}

impl Engine {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let event_loop = EventLoop::new();
        let window = create_window(&event_loop, width, height, title);
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
        let system_bind_group = render_system.render_window.device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("System Bind Group"),
            layout: &system_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry{
                    binding: 0,
                    resource: engine_time.time_buffer.as_entire_binding()
                }
            ]
        });

        Self {
            render_system,
            window,
            engine_time,
            event_loop,

            system_bind_group_layout,
            system_bind_group
        }
    }

    pub fn run(mut self) {
        self.engine_time.reset();
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => {
                    if !close_event_request_handler(event, control_flow)
                        && !self.render_system.resize_event_handler(event)
                    {
                        //Window events -- run through the list of input maps
                    }
                }
                Event::MainEventsCleared => {
                    if self.engine_time.update_time() {
                        self.engine_time
                            .update_buffer(&self.render_system.render_window.queue);
                        //Create a system that is going to hold the update loop
                        //Create a system that is going to hold the render loop
                        /*match self.render_system.render() {
                            Ok(_) => {
                                //End of frame
                                //Any entity deletion should happen here, this also means that if there is some sort
                                //of Scene object, it should be droped or created here too
                            }
                            // Reconfigure the surface if lost
                            Err(wgpu::SurfaceError::Lost) => self.render_system.configure_surface(),
                            // The system is out of memory, we should probably quit
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                *control_flow = ControlFlow::Exit
                            }
                            // All other errors (Outdated, Timeout) should be resolved by the next frame
                            Err(e) => eprintln!("{:?}", e),
                        }*/
                    }
                }
                _ => {}
            }
        });
    }
}
