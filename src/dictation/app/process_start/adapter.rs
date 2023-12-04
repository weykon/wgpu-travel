use super::super::state;

use super::super::common::Ready;

impl Ready for wgpu::Adapter {
    fn ready(app: &state::App) {
        Box::new(app.wgpu_instance.as_ref().unwrap().request_adapter(
            &wgpu::RequestAdapterOptions {
                compatible_surface: Some(app.surface.as_ref().unwrap()),
                ..Default::default()
            },
        ))
    }

    type Input = state::App;

    type Output=();
}
