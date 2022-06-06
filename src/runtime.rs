use crate::{Engine, EngineEvent};

pub trait Runtime {
    fn handle_event_queue<F>(&mut self, event_queue: &Vec<EngineEvent>, exit_event_loop: &mut F)
    where
        F: FnMut() -> ();
    fn update(&mut self, engine: &Engine);
    fn render(
        &mut self,
        engine: &mut Engine,
        screen_view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
    );
    fn frame_end<F>(&mut self, exit_event_loop: &mut F)
    where
        F: FnMut() -> ();
    fn before_exit(&mut self, engine: &Engine);
}
