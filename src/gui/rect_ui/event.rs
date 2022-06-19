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

pub enum UIEvent<'a> {
    Resize(UVec2),
    MouseButton(UIEventData<MouseInput>),

    /// Returns new mouse position
    MouseMove {
        corrected: UIEventData<Vec2>,
        raw: Vec2,
    },
    /// Returns mouse movement delta,
    MouseMoveDelta(UIEventData<DVec2>),

    CursorEnter,
    CursorExit,

    MouseWheel(UIEventData<Vec2>),
    KeyboardInput(UIEventData<KeyboardInput>),
    Update,
    Render {
        gui_rects: &'a mut GUIRects,
    },
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
                    Some(UIEvent::KeyboardInput(UIEventData::new(keyboard_input)))
                } else {
                    None
                }
            }
            //winit::event::WindowEvent::ModifiersChanged(_) => todo!(),
            winit::event::WindowEvent::CursorMoved { position, .. } => Some(UIEvent::MouseMove {
                corrected: UIEventData::<Vec2>::new(vec2(
                    position.x as f32,
                    size.y as f32 - position.y as f32,
                )),
                raw: vec2(position.x as f32, position.y as f32),
            }),
            winit::event::WindowEvent::MouseWheel { delta, .. } => {
                let d = match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => vec2(*x, *y),
                    winit::event::MouseScrollDelta::PixelDelta(pos) => {
                        vec2(pos.x as f32, pos.y as f32)
                    }
                };
                Some(UIEvent::MouseWheel(UIEventData::<Vec2>::new(d)))
            }
            winit::event::WindowEvent::MouseInput { state, button, .. } => Some(
                UIEvent::MouseButton(UIEventData::<MouseInput>::new(MouseInput {
                    button: *button,
                    state: *state,
                })),
            ),
            winit::event::WindowEvent::CursorEntered { .. } => {
                Some(UIEvent::CursorEnter)
            },
            winit::event::WindowEvent::CursorLeft { .. } => {
                Some(UIEvent::CursorExit)
            },
            //winit::event::WindowEvent::AxisMotion { device_id: (), axis: (), value: () }
            _ => None,
        },
        EngineEvent::ScaleFactorChanged { .. } => None,
        EngineEvent::DeviceEvent { device_id, event } => match event {
            winit::event::DeviceEvent::MouseMotion { delta } => Some(UIEvent::MouseMoveDelta(
                UIEventData::new(dvec2(delta.0, delta.1)),
            )),
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
