use winit::window::Window;

use winit::event_loop::EventLoop;

pub(crate) struct App {
    pub(crate) event_loop: Box<EventLoop<()>>,
    pub(crate) window: Box<Window>,
    pub(crate) wgpu_instance: Box<wgpu::Instance>,
    pub(crate) surface: Box<wgpu::Surface>,
    pub(crate) adapter: Box<(wgpu::Device, wgpu::Queue)>,
}

impl App {
    pub fn new() {
        App {
            event_loop: Box::new(None),
            window: Box::new(None),
            wgpu_instance: Box::new(None),
            surface: Box::new(None),
            adapter: Box::new(None),
        }
    }
}
