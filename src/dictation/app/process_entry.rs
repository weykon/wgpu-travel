use super::common::Ready;
use super::state::{self, App};
use winit::event_loop::EventLoop;
use winit::window::Window;
use super::running::Running;

pub(crate) async fn process() -> () {
    let app = state::App::new();
    App::ready(&app);
    let running = Running::run (&app);
    
}
mod adapter;
mod event_thing;
mod surface;
mod wgpu_instance;
mod window;

#[allow(unused_macros)]
macro_rules! ready_all {
    ($($field:ident),* $(,)?) => {
        $(
            self.$field = $field::ready();
        )*
    };
}

impl Ready for state::App {
    fn ready(app: &App) {
        app.event_loop = EventLoop::ready(app);
        app.window = Window::ready(app);
        app.wgpu_instance = wgpu::Instance::ready(app);
        app.surface = wgpu::Surface::ready(app);
        app.adapter = wgpu::Adapter::ready(app);
    }
    type Output = ();
}
