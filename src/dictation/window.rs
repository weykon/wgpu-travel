use super::my::Ready;
use winit::{event_loop::EventLoop, window::{Window, WindowBuilder}};

impl Ready for Window {
    type Input = EventLoop<()>;
    type Output = Window;

    fn ready(&self, event_loop: Self::Input) -> Self::Output {

    }
}
