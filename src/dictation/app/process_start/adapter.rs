use super::super::state;

use super::super::common::Ready;
use wgpu::Adapter;
use futures::executor::block_on;

impl Ready for Adapter {
    fn ready(app: Self::Input) -> Self::Output {
        let future = app
            .wgpu_instance
            .as_mut()
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(app.surface.as_ref()),
                ..Default::default()
            });
        let adapter = block_on(future).expect("Failed to request adapter");
        Box::new(adapter)
    }

    type Input = state::App;
    type Output = Box<wgpu::Adapter>;
}