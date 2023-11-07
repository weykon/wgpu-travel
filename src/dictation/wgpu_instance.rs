use crate::instance;

use super::my::Ready;

impl Ready for wgpu::Instance {
    type Input = ();
    type Output = wgpu::Instance;

    fn ready(&self, _input: Self::Input) -> Self::Output {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        instance
    }
}

