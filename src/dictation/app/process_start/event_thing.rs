use super::super::state;

use winit::event_loop::EventLoop;

use super::super::common::Ready;

impl Ready for EventLoop {
    fn ready(app: &state::App) {
        Box::new(EventLoop::new())
    }
}

