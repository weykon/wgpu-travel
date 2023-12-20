use std::iter;

use super::Rendering;
use crate::app::{config, state::App};

impl Rendering for App {
    type Input = ();
    type Output = Result<(), wgpu::SurfaceError>;

    fn render(&self, input: Self::Input) -> Self::Output {
        let output = self.surface.as_mut().unwrap().get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let config = self.config.unwrap();

        let encoder = config
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        let clear_color = wgpu::Color::BLACK;
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear_color),
                        store: true,
                    },
                })],
                ..Default::default()
            });
        }

        config.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
