use wgpu::{Instance, Surface};

use crate::{atom::adapter::AdapterStorage, ready::ReadyStatic};

impl ReadyStatic<(Instance, Surface), Self> for AdapterStorage {
    fn ready(inst_and_surface: (Instance, Surface)) -> Self {
        let adapter = futures::executor::block_on(inst_and_surface.0.request_adapter(
            &wgpu::RequestAdapterOptions {
                compatible_surface: Some(&inst_and_surface.1),
                ..Default::default()
            },
        ))
        .unwrap();
        let (device, queue) = futures::executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None,
        ))
        .unwrap();

        AdapterStorage {
            device,
            queue,
            adapter,
        }
    }
}
