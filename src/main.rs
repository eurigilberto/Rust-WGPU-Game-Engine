use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn configure_window(window: &winit::window::Window){
    window.set_title("WGPU_RUST");
    window.set_inner_size(winit::dpi::LogicalSize::new(720.0,720.0));
    /*window
        .set_cursor_grab(true)
        .expect("Windows does not support cursor Grab");*/
    //window.set_cursor_visible(false);
    //window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
}

fn main() {
    println!("Hello, world!");
}