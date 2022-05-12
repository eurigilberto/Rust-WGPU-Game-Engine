pub use glam;
mod render_system;
mod engine_time;
mod gui;
mod font;
mod color;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn create_window(event_loop: &EventLoop<()>, width:u32, height: u32, title: &str) -> winit::window::Window {
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

pub struct Engine{
    pub render_system: render_system::RenderSystem,
    pub window: winit::window::Window,
    pub event_loop: EventLoop<()>
}

impl Engine{
    pub fn new(width:u32, height: u32, title: &str) -> Self{
        let event_loop = EventLoop::new();
        let window = create_window(&event_loop, width, height, title);
        let render_system = pollster::block_on(render_system::RenderSystem::new(&window));
        Self{
            render_system,
            window,
            event_loop
        }
    }

    pub fn run(mut self){
        let mut engine_time = engine_time::EngineTime::new(11, &self.render_system);

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
                    if engine_time.update_time() {
                        engine_time.update_buffer(&self.render_system.render_window.queue);
                        match self.render_system.render() {
                            Ok(_) => {}
                            // Reconfigure the surface if lost
                            Err(wgpu::SurfaceError::Lost) => self.render_system.configure_surface(),
                            // The system is out of memory, we should probably quit
                            Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                            // All other errors (Outdated, Timeout) should be resolved by the next frame
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }
                }
                _ => {}
            }
        });
    }
}
