use crate::ready::ReadyStatic;
use wgpu::{Backends, Instance, InstanceDescriptor};

impl ReadyStatic<(), Instance> for Instance {
    fn ready(_: ()) -> Self {
        Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        })
    }
}
