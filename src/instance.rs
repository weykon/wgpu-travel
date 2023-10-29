use bytemuck;
pub struct Instance {
    position: glam::Vec3,
    rotation: glam::Quat,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceRaw {
    model: [[f32; 4]; 4],
}
