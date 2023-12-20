use super::config::ConfigStorage;
use super::picture::Picture;
use super::rendering::render;
use super::running::Running;
use crate::dictation::event;
use wgpu::Adapter;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

pub struct App {
    pub event_loop: Box<Option<EventLoop<()>>>,
    pub window: Box<Option<Window>>,
    pub wgpu_instance: Box<Option<wgpu::Instance>>,
    pub surface: Box<Option<wgpu::Surface>>,
    pub adapter: Box<Option<Adapter>>,
    pub pictures: Vec<Picture>,
    pub running: Box<Option<Running>>,
    pub config: Option<ConfigStorage>,
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
            running: Box::new(None),
            config: None,
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        if self.running.is_some() {
            return self
                .running
                .as_mut()
                .is_some_and(|mut x| x.controller.process_events(event));
        }
        return false;
    }

    pub fn pre_data_update(&self) {}

    pub fn handle_events(
        &self,
        event_loop: &EventLoop<()>,
        window: &winit::window::Window,
        app_state: &App,
    ) {
        let run = event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                // 这里这个取false过的，是如果不是摄像头移动就考虑接下来的东西。
                // 这里里面说的是wsad按键的接受
                if !self.input(event) {
                    // 我在想如何在这里的代码整理一下，不用那么长，起码分一下策略什么的。
                    event::handle::input::handle_any_input(
                        event,
                        control_flow,
                        &mut app_state,
                        &window,
                    );
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                self.pre_data_update();
                match render(&mut self) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => self
                        .running
                        .as_mut()
                        .unwrap()
                        .resize(&mut self, self.running.as_mut().unwrap().window_size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // 除非我们手动请求，RedrawRequested 将只会触发一次。
                window.request_redraw();
            }
            _ => {}
        });
    }
}
