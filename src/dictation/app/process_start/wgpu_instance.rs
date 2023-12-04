use super::super::state;

use super::super::common::Ready;

impl Ready for wgpu::Instance {
    fn ready(app: &state::App) {
        Box::new(wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        }))
    }
    type Input = state::App;

    type Output=();
}

