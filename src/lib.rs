use std::{collections::VecDeque, time::SystemTime};

use engine::time::Microsecond;
pub use glam;
pub mod color;
pub mod font;
pub mod graphics;
pub mod gui;
pub mod math_utils;
pub use bytemuck;
use glam::{uvec2, UVec2};
pub use half;
pub use wgpu;
pub use winit;
pub mod engine;
pub use engine::Engine;
pub use slotmap;
pub mod entity_component;
pub mod runtime;
pub use rand;
pub use runtime::Runtime;
pub use uuid;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub fn default_close_event_handler<F>(event: &EngineEvent, exit_event_loop: &mut F) -> bool
where
    F: FnMut() -> (),
{
    if let EngineEvent::WinitEvent(e) = event {
        match e {
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
                exit_event_loop();
                true
            }
            _ => false,
        }
    } else {
        false
    }
}

pub fn render<R: 'static + Runtime>(
    engine: &mut Engine,
    runtime: &mut R,
) -> Result<Microsecond, wgpu::SurfaceError> {
    let output: wgpu::SurfaceTexture = engine
        .graphics
        .render_window
        .surface
        .get_current_texture()?;

    if output.suboptimal {
        println!("Suboptimal surface!!");
    }

    let screen_view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder =
        engine
            .graphics
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

    runtime.render(engine, &screen_view, &mut encoder);

    let gpu_lock_time_start = std::time::Instant::now();
    let mut command_buffers = Vec::<wgpu::CommandBuffer>::new();
    command_buffers.push(encoder.finish());
    engine.graphics.queue.submit(command_buffers);
    output.present();
    engine.graphics.device.poll(wgpu::Maintain::Wait);
    //pollster::block_on(on_gpu_done);

    Ok(Microsecond(gpu_lock_time_start.elapsed().as_micros()))
}

#[derive(Debug)]
pub enum EngineEvent {
    WinitEvent(winit::event::WindowEvent<'static>),
    ScaleFactorChanged {
        scale_factor: f64,
        new_inner_size: UVec2,
    },
    DeviceEvent {
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    },
}

fn is_resize_event(event: &winit::event::WindowEvent) -> bool {
    if let WindowEvent::ScaleFactorChanged { .. } | WindowEvent::Resized(..) = event {
        return true;
    }
    return false;
}

pub enum WindowOrDeviceEvent<'a> {
    Window(winit::event::WindowEvent<'a>),
    Device(DeviceId, winit::event::DeviceEvent),
}

fn remove_any_resize_event(event_queue: &mut VecDeque<EngineEvent>) {
    let mut found_resize = false;
    let mut remove_index = 0;
    for (index, ee) in event_queue.into_iter().enumerate() {
        if let EngineEvent::ScaleFactorChanged { .. }
        | EngineEvent::WinitEvent(WindowEvent::Resized(..)) = ee
        {
            remove_index = index;
            found_resize = true;
            break;
        }
    }
    if found_resize {
        event_queue.remove(remove_index);
    }
}

fn push_window_event(
    event_queue: &mut VecDeque<EngineEvent>,
    window_event: winit::event::WindowEvent,
) {
    if let WindowEvent::ScaleFactorChanged {
        scale_factor,
        new_inner_size,
    } = window_event
    {
        event_queue.push_back(EngineEvent::ScaleFactorChanged {
            scale_factor: scale_factor,
            new_inner_size: uvec2(new_inner_size.width, new_inner_size.height),
        });
    } else {
        if let Some(static_event) = window_event.to_static() {
            event_queue.push_back(EngineEvent::WinitEvent(static_event));
        }
    }
}

fn push_event(event_queue: &mut VecDeque<EngineEvent>, event: WindowOrDeviceEvent) {
    if event_queue.len() == event_queue.capacity() {
        event_queue.pop_front();
    }
    match event {
        WindowOrDeviceEvent::Window(window_event) => {
            if is_resize_event(&window_event) {
                remove_any_resize_event(event_queue);
            }
            push_window_event(event_queue, window_event);
        }
        WindowOrDeviceEvent::Device(device_id, event) => {
            event_queue.push_back(EngineEvent::DeviceEvent { device_id, event });
        }
    }
}

pub fn start_engine_loop<R: 'static + Runtime>(
    mut engine: Engine,
    mut runtime: R,
    event_loop: EventLoop<()>,
) {
    engine.timer.reset();
    let mut first_frame = true;
    let mut event_queue = VecDeque::<EngineEvent>::with_capacity(100);
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::LoopDestroyed => {
                runtime.before_exit(&mut engine);
            }
            Event::WindowEvent { event, window_id } if window_id == runtime.get_window_id() => {
                push_event(&mut event_queue, WindowOrDeviceEvent::Window(event));
            }
            Event::DeviceEvent { device_id, event } => {
                push_event(
                    &mut event_queue,
                    WindowOrDeviceEvent::Device(device_id, event),
                );
            }
            Event::MainEventsCleared => {
                if engine.timer.update_time() {
                    let mut close_app = || {
                        *control_flow = ControlFlow::Exit;
                    };

                    let frame_start_time = std::time::Instant::now();
                    runtime.frame_start(&engine);
                    engine.operation_timer.frame_start_time =
                        Microsecond(frame_start_time.elapsed().as_micros());

                    engine.timer.update_buffer(&engine.graphics.queue);

                    let event_handling_time = std::time::Instant::now();
                    runtime.handle_event_queue(&event_queue, &mut engine, &mut close_app);
                    engine.operation_timer.event_handling_time =
                        Microsecond(event_handling_time.elapsed().as_micros());

                    event_queue.clear();

                    let update_time = std::time::Instant::now();
                    runtime.update(&engine, &mut close_app);
                    engine.operation_timer.update_time =
                        Microsecond(update_time.elapsed().as_micros());

                    let render_time = std::time::Instant::now();
                    let render_result = render(&mut engine, &mut runtime);
                    engine.operation_timer.render_time =
                        Microsecond(render_time.elapsed().as_micros());

                    match render_result {
                        Ok(gpu_lock_time) => {
                            //println!("GPU LOCK TIME {}", gpu_lock_time * 1000.0);
                            engine.operation_timer.gpu_lock_time = gpu_lock_time;
                            let operation_time = std::time::Instant::now();
                            runtime.frame_end(&mut engine, &mut close_app);
                            engine.operation_timer.frame_end_time =
                                Microsecond(operation_time.elapsed().as_micros());
                        }
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => engine.graphics.configure_surface(),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        //
                        Err(wgpu::SurfaceError::Outdated) => {
                            println!("Outdated Surface!");
                            engine.graphics.configure_surface()
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
