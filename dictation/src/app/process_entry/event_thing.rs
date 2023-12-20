use crate::app::state::App;
use winit::event_loop::EventLoop;
use super::super::common::Ready;

impl Ready for EventLoop<()> {
    fn ready(app: &App) -> Self::Output {
        Box::new(Some(EventLoop::new()))
    }
    type Output = Box<Option<EventLoop<()>>>;
}
