use super::{event::handle_events, my::Ready, state::AppState};
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub fn run() {
    pollster::block_on(process());
}

async fn process() -> () {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let app_state = AppState::new(&window);

    let wgpu_instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });
    let surface: wgpu::Surface = unsafe { wgpu_instance.create_surface(&window).unwrap() };
    let adapter = wgpu_instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await
        .unwrap();
    // 命令队列 queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // 追踪 API 调用路径
        )
        .await
        .unwrap();
}
