use std::{borrow::BorrowMut, cell::RefCell};

use wgpu::{StoreOp, Surface};
use winit::{
    event::{KeyboardInput, MouseButton, WindowEvent},
    window::Window,
};

use crate::scheduler::Scheduler;

use super::{adapter::AdapterStorage, event_storage::EventStorage};

pub struct App {
    pub event_storage: RefCell<EventStorage>,
    pub window: RefCell<Window>,
    pub surface: RefCell<Surface>,
    pub adapter_storage: Option<Box<AdapterStorage>>,
    pub surface_config: Option<wgpu::SurfaceConfiguration>,
    pub scheduler: Box<Option<Scheduler>>,
    pub size: Option<winit::dpi::PhysicalSize<u32>>,
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

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            match self.surface_config {
                Some(ref mut config) => {
                    config.width = new_size.width;
                    config.height = new_size.height;
                    self.surface
                        .borrow_mut()
                        .configure(&self.adapter_storage.unwrap().device, config);
                }
                None => {}
            }
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let ready_draw_texture_from_now_surface =
            self.surface.borrow().get_current_texture().unwrap();

        let view =
            ready_draw_texture_from_now_surface
                .texture
                .create_view(&wgpu::TextureViewDescriptor {
                    label: None,
                    format: None,
                    mip_level_count: Some(1),
                    dimension: None,
                    aspect: wgpu::TextureAspect::All,
                    base_mip_level: 0,
                    base_array_layer: 0,
                    array_layer_count: None,
                });

        let mut encoder = self.adapter_storage.unwrap().device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        );

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                timestamp_writes: None,
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
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
            });
            let render_pipeline = self.scheduler.borrow_mut().pipeline_layouts.unwrap().storage[0]
                .render_pipeline
                .as_ref()
                .unwrap();

            render_pass.set_pipeline(&render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as _);
        }

        Ok(())
    }
}
