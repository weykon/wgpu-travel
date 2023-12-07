use super::picture::Picture;
use wgpu::Adapter;
use winit::event_loop::EventLoop;
use winit::window::Window;

pub struct App {
    pub event_loop: Box<Option<EventLoop<()>>>,
    pub window: Box<Option<Window>>,
    pub wgpu_instance: Box<Option<wgpu::Instance>>,
    pub surface: Box<Option<wgpu::Surface>>,
    pub adapter: Box<Option<Adapter>>,
    pub pictures: Vec<Picture>,
}

impl App {
    pub fn new() -> Self {
        App {
            event_loop: Box::new(None),
            window: Box::new(None),
            wgpu_instance: Box::new(None),
            surface: Box::new(None),
            adapter: Box::new(None),
            pictures: vec![],
        }
    }
}

