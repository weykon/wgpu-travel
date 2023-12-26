use std::cell::RefCell;

use wgpu::{Surface};
use winit::{
    event::{KeyboardInput, MouseButton, WindowEvent},
    window::Window,
};

use super::{event_storage::EventStorage, adapter::AdapterStorage};


pub struct App {
    pub event_storage: RefCell<EventStorage>,
    pub window: RefCell<Window>,
    pub surface: RefCell<Surface>,
    pub adapter_storage:  Option<Box<AdapterStorage>>
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
            } => true,
            WindowEvent::MouseWheel { delta, .. } => true,
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => true,
            _ => false,
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) {
        let ready_draw_texture_from_now_surface =
            self.surface.borrow().get_current_texture().unwrap();
        let view = ready_draw_texture_from_now_surface
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

    }
}
