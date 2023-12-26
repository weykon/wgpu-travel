use std::cell::RefCell;

use winit::{
    event::{KeyboardInput, WindowEvent, MouseButton},
    window::Window,
};

use super::event_storage::EventStorage;

pub struct App {
    pub event_storage: RefCell<EventStorage>,
    pub window: RefCell<Window>,
}
impl App {
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => {
                true
            },
            WindowEvent::MouseWheel { delta, .. } => {
                true
            }
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => {
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {}
    
}
