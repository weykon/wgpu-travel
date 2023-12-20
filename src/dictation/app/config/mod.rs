use self::device::DeviceConfig;

use super::state::App;

pub mod device;
pub mod surface;
// 专门来收集和处理config的

pub trait Config {
    /// 类似于default吧，但是我给它强调如果他无差异性的话，在硬编码当中也是好理解的。
    type Input;
    type Output;
    fn fixed(&self, input: &mut Self::Input) -> Self::Output;
}

pub struct ConfigStorage {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter: wgpu::Adapter,
    pub surface: wgpu::SurfaceConfiguration,
}

pub fn config(mut app: App) -> App {
    let device_config = DeviceConfig::default();
    device_config.fixed(&mut app);

    let surface = app.surface.take();
    if let Some(surface) = surface {
        surface.fixed(&mut app);
    }

    
    return app;
}
