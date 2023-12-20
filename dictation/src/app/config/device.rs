use crate::app::state::App;

use super::Config;

#[derive(Default)]
pub struct DeviceConfig {
    pub device: Option<wgpu::Device>,
    pub queue: Option<wgpu::Queue>,
    pub adapter: Option<wgpu::Adapter>,
}

impl Config for DeviceConfig {
    type Input = App;
    type Output = DeviceConfig;

    fn fixed(&self, app: &mut Self::Input) -> Self::Output {
        let adapter = app.adapter.as_mut().unwrap();
        let a_future = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // 追踪 API 调用路径
        );
        let (device, queue) = match futures::executor::block_on(a_future) {
            Ok(x) => x,
            Err(e) => panic!("Error: {}", e),
        };

        DeviceConfig {
            device: Some(device),
            queue: Some(queue),
            adapter: Some(adapter),
        }
    }
}
