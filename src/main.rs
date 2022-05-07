use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod render_system;

fn create_window(event_loop: &EventLoop<()>) -> winit::window::Window {
    let window = WindowBuilder::new()
        .with_decorations(true)
        .build(event_loop)
        .unwrap();
    window.set_title("Rust_engine");
    window.set_inner_size(winit::dpi::LogicalSize::new(720.0, 720.0));
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

fn main() {
    let event_loop = EventLoop::new();
    let window = create_window(&event_loop);

    // State::new uses async code, so we're going to wait for it to finish
    let mut render_system = pollster::block_on(render_system::RenderSystem::new(&window));
    let mut last_render_time = std::time::Instant::now();
    let mut acummulated_time = 0;
    // main()
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !close_event_request_handler(event, control_flow)
                    && !render_system.resize_event_handler(event)
                {
                    //Window events -- run through the list of input maps
                }
            }
            Event::MainEventsCleared => {
                let now = std::time::Instant::now();
                let dt = now - last_render_time;
                acummulated_time += dt.as_millis();
                if acummulated_time > 11 {
                    last_render_time = now;

                    let delta_time = acummulated_time as f32 / 1000.0;

                    //Call an update request with
                    //delta_time

                    match render_system.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => render_system.configure_surface(),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }

                    acummulated_time = 0;
                }
            }
            _ => {}
        }
    });
}
