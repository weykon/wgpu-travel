use crate::dictation::app::state::App;

use super::super::common::Ready;

impl Ready for wgpu::Instance {
    fn ready(app: &App) -> Self::Output {
        Box::new(Some(wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        })))
    }

    type Output = Box<Option<wgpu::Instance>>;
}
