use crate::dictation::app::state::App;
use super::super::common::Ready;
use futures::executor::block_on;
use wgpu::Adapter;

impl Ready for Adapter {
    fn ready(app: &App) -> Self::Output {
        let inst = app.wgpu_instance.as_mut().unwrap();
        let surface = app.surface.as_mut().unwrap();
        let future = inst.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        });

        let adapter = block_on(inst.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }));

        Box::new(adapter)
    }

    type Output = Box<Option<Adapter>>;
}

