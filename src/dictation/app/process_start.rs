use super::common::Ready;
use super::state;
use winit::event_loop::EventLoop;
use winit::window::{Window};

pub(crate) async fn process() -> () {
    let mut app = state::App::new();

    app.ready();
}
mod event_thing;
mod window;
mod wgpu_instance;
mod surface;
mod adapter;
impl Ready for state::App {
    fn ready(&self) {
        self.event_loop = EventLoop::ready(&self);
        self.window = Window::ready(&self);
        self.wgpu_instance = wgpu::Instance::ready(&self);
        self.surface = wgpu::Surface::ready(&self);
        self.adapter =  wgpu::Adapter::ready(&self);
    }
}
