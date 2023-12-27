use wgpu::Surface;
use winit::window::Window;

use crate::atom::adapter::AdapterStorage;

use super::Config;

impl Config<(&AdapterStorage, &Window), ()> for Surface {
    fn config(&mut self, input: (&AdapterStorage, &Window)) {
        let caps = self.get_capabilities(&input.0.adapter);
        let size = input.1.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };
        self.configure(&input.0.device, &config);
    }
}
