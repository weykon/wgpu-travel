use std::{borrow::Borrow, cell::RefCell};

use wgpu::Surface;
use winit::{
    event::{KeyboardInput, MouseButton, WindowEvent},
    window::Window,
};

use super::{adapter::AdapterStorage, event_storage::EventStorage};

pub struct App {
    pub event_storage: RefCell<EventStorage>,
    pub window: RefCell<Window>,
    pub surface: RefCell<Surface>,
    pub adapter_storage: Option<Box<AdapterStorage>>,
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

        let mut encoder = self.adapter_storage.unwrap().device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        );

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }
    }
}
