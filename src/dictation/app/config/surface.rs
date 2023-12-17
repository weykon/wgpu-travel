use super::super::state::App;
use wgpu::Surface;

use super::Config;

impl Config for Surface {
    type Input = &'static App;
    type Output = wgpu::SurfaceConfiguration;
    fn fixed(&self, app: Self::Input) -> Self::Output {
        let adapter = app.adapter.as_mut().unwrap();
        let caps = self.get_capabilities(&adapter);
        let size = app.window.as_mut().unwrap().inner_size();

        // 以下有很多不知含义的变量，我将其它们都蒙蔽掉，用这里的变量去一一辟出意义
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };

        config
    }
}
