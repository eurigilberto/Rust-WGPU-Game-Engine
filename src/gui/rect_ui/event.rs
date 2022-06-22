use crate::EngineEvent;
use glam::{dvec2, vec2, DVec2, UVec2, Vec2};

use super::GUIRects;

pub struct MouseInput {
    pub button: winit::event::MouseButton,
    pub state: winit::event::ElementState,
}

pub struct KeyboardInput {
    pub key: winit::event::VirtualKeyCode,
    pub state: winit::event::ElementState,
}

pub struct ExtraRenderSteps {
    render_steps: Vec<(Box<dyn FnOnce(&mut GUIRects) -> ()>, u32)>,
}
impl ExtraRenderSteps {
    pub fn new(capacity: usize) -> Self {
        Self {
            render_steps: Vec::with_capacity(capacity),
        }
    }
    pub fn push(&mut self, render_step: Box<dyn FnOnce(&mut GUIRects) -> ()>, depth: u32) {
        self.render_steps.push((render_step, depth));
    }
    pub fn execute_render_steps(&mut self, gui_rects: &mut GUIRects) {
        self.render_steps
            .sort_by(|(_, a_depth), (_, b_depth)| a_depth.cmp(b_depth));

        for steps in self.render_steps.drain(..) {
            steps.0(gui_rects);
        }
    }
}

pub enum UIEvent<'a> {
    Resize(UVec2),
    MouseButton(MouseInput),

    /// Returns new mouse position
    MouseMove {
        corrected: Vec2,
        raw: Vec2,
    },
    /// Returns mouse movement delta,
    MouseMoveDelta(DVec2),

    CursorEnter,
    CursorExit,

    MouseWheel(Vec2),
    KeyboardInput(KeyboardInput),
    Update,
    Render {
        gui_rects: &'a mut GUIRects,
        extra_render_steps: ExtraRenderSteps,
    },
    Consumed,
}

impl UIEvent<'_> {
    pub fn get_name(&self) -> &str {
        match self {
            UIEvent::Resize(..) => "Resize",
            UIEvent::MouseButton(..) => "Mouse Button",
            UIEvent::MouseMove { .. } => "Mouse Move",
            UIEvent::MouseMoveDelta(..) => "Mouse Move Delta",
            UIEvent::CursorEnter => "Cursor Enter",
            UIEvent::CursorExit => "Cursor Exit",
            UIEvent::MouseWheel(..) => "Mouse Wheel",
            UIEvent::KeyboardInput(..) => "Keyboard Input",
            UIEvent::Update => "Update",
            UIEvent::Render { .. } => "Render",
            UIEvent::Consumed => "Consumed",
        }
    }

    pub fn consume(&mut self) {
        *self = Self::Consumed;
    }
}

pub struct UIEventData<T> {
    pub data: T,
    used: bool,
}

impl<T> UIEventData<T> {
    pub fn new(data: T) -> Self {
        Self { data, used: false }
    }

    pub fn use_event(&mut self) {
        self.used = true;
    }

    pub fn get_used(&self) -> bool {
        self.used
    }
}

#[derive(Debug)]
pub enum GUIState {
    Active,
    Hovered,
    Inactive,
}

pub fn default_event_transformation(event: &EngineEvent, size: UVec2) -> Option<UIEvent> {
    match event {
        EngineEvent::WinitEvent(event) => match event {
            winit::event::WindowEvent::KeyboardInput { input, .. } => {
                if let Some(keycode) = input.virtual_keycode {
                    let keyboard_input = KeyboardInput {
                        key: keycode,
                        state: input.state,
                    };
                    Some(UIEvent::KeyboardInput(keyboard_input))
                } else {
                    None
                }
            }
            //winit::event::WindowEvent::ModifiersChanged(_) => todo!(),
            winit::event::WindowEvent::CursorMoved { position, .. } => Some(UIEvent::MouseMove {
                corrected: vec2(position.x as f32, size.y as f32 - position.y as f32),
                raw: vec2(position.x as f32, position.y as f32),
            }),
            winit::event::WindowEvent::MouseWheel { delta, .. } => {
                let d = match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => vec2(*x, *y),
                    winit::event::MouseScrollDelta::PixelDelta(pos) => {
                        vec2(pos.x as f32, pos.y as f32)
                    }
                };
                Some(UIEvent::MouseWheel(d))
            }
            winit::event::WindowEvent::MouseInput { state, button, .. } => {
                Some(UIEvent::MouseButton(MouseInput {
                    button: *button,
                    state: *state,
                }))
            }
            winit::event::WindowEvent::CursorEntered { .. } => Some(UIEvent::CursorEnter),
            winit::event::WindowEvent::CursorLeft { .. } => Some(UIEvent::CursorExit),
            //winit::event::WindowEvent::AxisMotion { device_id: (), axis: (), value: () }
            _ => None,
        },
        EngineEvent::ScaleFactorChanged { .. } => None,
        EngineEvent::DeviceEvent { device_id, event } => match event {
            winit::event::DeviceEvent::MouseMotion { delta } => {
                Some(UIEvent::MouseMoveDelta(dvec2(delta.0, delta.1)))
            }
            _ => None,
        },
    }
}

//To be implemented
//winit::event::WindowEvent::CursorEntered { device_id } => todo!(),
//winit::event::WindowEvent::CursorLeft { device_id } => todo!(),
//winit::event::WindowEvent::DroppedFile(_) => todo!(),
//winit::event::WindowEvent::HoveredFile(_) => todo!(),
//winit::event::WindowEvent::HoveredFileCancelled => todo!(),
//winit::event::WindowEvent::ReceivedCharacter(_) => todo!(),
//winit::event::WindowEvent::Focused(_) => todo!(),
/*winit::event::WindowEvent::TouchpadPressure {
    device_id,
    pressure,
    stage,
} => todo!(),*/
/*winit::event::WindowEvent::AxisMotion {
    device_id,
    axis,
    value,
} => todo!(),*/
//winit::event::WindowEvent::Touch(_) => todo!(),
