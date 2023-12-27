use wgpu::{RenderPass, CommandEncoder, TextureView};

use super::ReadyStatic;

impl<'pass> ReadyStatic<(CommandEncoder, TextureView), RenderPass<'pass>> for RenderPass<'pass> {
    fn ready(mut input: (CommandEncoder, TextureView)) -> RenderPass<'pass> {
        input.0.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &input.1,
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
        })
    }
}
