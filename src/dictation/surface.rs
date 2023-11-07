use super::my::Ready;

pub struct MySurface {
    window: winit::window::Window,
}

impl Ready for MySurface {
    type Input = wgpu::Instance;
    type Output = wgpu::Surface;

    fn ready(&self, instance: Self::Input) -> Self::Output {
        unsafe { instance.create_surface(&self.window).unwrap() }
    }
}
