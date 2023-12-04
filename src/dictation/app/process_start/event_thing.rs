use super::super::state;

use winit::event_loop::EventLoop;

use super::super::common::Ready;

impl Ready for EventLoop<()> {
    fn ready(_: Self::Input) -> Self::Output {
        Box::new(EventLoop::new())
    }
    type Input = state::App;
    type Output = Box<EventLoop<()>>;
}
