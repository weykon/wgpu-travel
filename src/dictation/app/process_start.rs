use super::common::Ready;
use super::state;
use winit::event_loop::EventLoop;
use winit::window::Window;

pub(crate) async fn process() -> () {
    let mut app = state::App::new();
    app.ready();
}
mod adapter;
mod event_thing;
mod surface;
mod wgpu_instance;
mod window;

impl Ready for state::App {
    fn ready(&self) {
        // self.event_loop = EventLoop::ready(&self);
        // self.window = Window::ready(&self);
        // self.wgpu_instance = wgpu::Instance::ready(&self);
        // self.surface = wgpu::Surface::ready(&self);
        // self.adapter =  wgpu::Adapter::ready(&self);
    
        fn ready(&mut self) {
            ready_all!(event_loop, window, wgpu_instance, surface, adapter);
        }
    }

    type Input = state::App;

    type Output = ();
}

macro_rules! ready_all {
    ($($field:ident),* $(,)?) => {
        $(
            self.$field = $field::ready(&self);
        )*
    };
}
